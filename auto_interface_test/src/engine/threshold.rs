use std::{fs, fs::{File, OpenOptions}, process::{Command, Output}, path::{Path, PathBuf}, 
    io::{BufRead, Write, BufReader}, collections::HashMap};
use serde::{Serialize, Deserialize};
use serde_json;
use encoding_rs_io::{self, DecodeReaderBytesBuilder};
use encoding_rs;
use lazy_static::lazy_static;
use std::sync::Mutex;

#[derive(Debug, Deserialize, Serialize)]
struct Value {
    path: Box<PathBuf>,
    entropy: f32,
    chi_square: f32,
    mean: f32,
    monte_carlo_pi: f32,
    serial_correlation: f32
}

#[derive(Deserialize, Serialize, Debug)]
struct Sys {
    syscall: String,
    arg1: String,
    arg2: String,
    arg3: String,
}

lazy_static! {
    static ref OPEND_FILES: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

fn ent_compute(file_path: &String) {
    
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

    let dir = "files/tmp_files".to_string();
    if !Path::new(dir.as_str()).exists() {
        fs::create_dir_all(dir.as_str()).unwrap();
    }

    match sys.syscall.as_str() {
        "open"|"openat" => {
            let file_name = sys.arg3.clone();
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
                    let tmp = OPEND_FILES.lock().unwrap();
                    let file_path = tmp.get(&sys.arg1).unwrap();

                    ent_compute(file_path);

                    OPEND_FILES.lock().unwrap()
                        .remove(&sys.arg1);
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
        }
    }
}

pub fn threshold_analysis(line: String) {
    syscall_separate_th(line);
}
