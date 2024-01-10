use std::{fs, path::{Path, PathBuf}, collections::HashMap};
use std::io::{BufRead, Write, BufReader};
use serde::{Serialize, Deserialize};
use serde_json;
use encoding_rs_io::{self, DecodeReaderBytesBuilder};
use lazy_static::lazy_static;
use std::sync::Mutex;
use strum::IntoEnumIterator;

use super::computer::ent_compute;
use super::ValueType;

#[derive(Deserialize, Serialize, Debug)]
struct Sys {
    syscall: String,
    arg1: String,
    arg2: String,
    arg3: String,
}

lazy_static! {
    #[derive(Debug)]
    static ref OPEND_FILES: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

fn syscall_separate_th(line: String) {
    if line.contains("probes") {
        return;
    }

    let line_sj: serde_json::Value = serde_json::from_str(line.as_str()).unwrap();
    let mut data = line_sj["data"].to_string();
    data.remove(0);                 // remove \" before the line
    data.remove(data.len() - 1);    // remove \" at the end of the line
    
    let v: Vec<&str> = data.splitn(4, " ").collect();
    let sys = Sys {
        syscall: v[0].to_string(),
        arg1: v[1].to_string(),   // fd
        arg2: v[2].to_string(),   // Ret  open-> _
        arg3: v[3].to_string()    // Data/Path 
    };

    let dir = "build/tmp_files".to_string();
    if !Path::new(dir.as_str()).exists() {
        fs::create_dir_all(dir.as_str()).unwrap();
    }

    match sys.syscall.as_str() {
        "open"|"openat" => {
            let file_name = sys.arg3.clone();
            // println!("file name{}", file_name);
            if file_name.len() != 0 {
                let v: Vec<&str> = file_name.split("/").collect();
                let file_path = dir.clone() + "/" + v[v.len() - 1];
                OPEND_FILES.lock().unwrap()
                    .insert(
                        sys.arg1.clone(), 
                        file_path);
            }
        },
        "close" => {
            if OPEND_FILES.lock().unwrap()
                .contains_key(&sys.arg1) {
                    let mut tmp = OPEND_FILES.lock().unwrap();
                    if let Some(file_path) = tmp.remove(&sys.arg1) {
                        if Path::new(&file_path).exists() {
                            let values: Vec<f32> = ent_compute(&file_path);
                            output_to_files(values);
                        }
                        
                    }        
            }
        },
        _ => {
            let buf_len = sys.arg2.parse::<u32>().unwrap();
            if OPEND_FILES.lock().unwrap()
                .contains_key(&sys.arg1) {
                if buf_len != 0 {
                    let tmp = OPEND_FILES.lock().unwrap();
                    let file_path = tmp.get(&sys.arg1).unwrap();
    
                    if !Path::new(file_path.as_str()).exists() {
                        let _result = fs::File::create(file_path.as_str()).unwrap();
                    }
                    let mut file = fs::OpenOptions::new()
                        .write(true)
                        .append(true)
                        .open(file_path.as_str())
                        .unwrap();
                    file.write(sys.arg3.as_bytes()).unwrap();
                }
            }
            // println!("other: {}", sys.arg3);
        }
    }
}

fn output_to_files(values: Vec<f32>) {
    let dir = "build/threshold_output".to_string();
    if !Path::new(dir.as_str()).exists() {
        fs::create_dir_all(dir.as_str()).unwrap();
    }

    let v_types: Vec<&str> = vec!("FileBytes", "Entropy", "ChiSquare", "Mean", 
                                 "MontecarloPi", "SerialCorrelation", "_LAST_");
    
    for i in 0 .. values.len() {
        let file_path = dir.clone() + "/" + v_types[i];
        if !Path::new(file_path.as_str()).exists() {
            let _result = fs::File::create(file_path.as_str()).unwrap();
        }
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(file_path.as_str())
            .unwrap();
        file.write((values[i].to_string() + "\n").as_bytes()).unwrap();
    }
}

pub fn threshold_analysis(line: String) {
    syscall_separate_th(line);
}
