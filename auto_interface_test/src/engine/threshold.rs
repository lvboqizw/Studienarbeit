use std::fs; 
use std::sync::Mutex;
use std::path::Path;
use std::collections::HashMap;
use std::io::Write;
use serde_json;
use lazy_static::lazy_static;

use super::{computer::ent_compute, ValueType};
use serde::{Serialize, Deserialize};
use serde_json::Result;


#[derive(Debug)]
struct Sys {
    syscall: String,
    arg1: String,
    arg2: String,
    arg3: String,
}

#[derive(Serialize, Deserialize)]
struct Res {
    file: String,
    entropy: f32,
    chisquare: f32,
    mean: f32,
    montecarlo: f32,
    serial: f32,
}

static mut _N: i32 = 0;

lazy_static! {
    #[derive(Debug)]
    static ref OPEND_FILES: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

pub fn threshold_analysis(line: String) {
    if line.contains("probes") {
        return;
    }

    let line_sj: serde_json::Value = serde_json::from_str(line.as_str()).unwrap();
    let mut data = line_sj["data"].to_string();
    data.remove(0);                 // remove \" before the line
    data.remove(data.len() - 1);    // remove \" at the end of the line
    
    let v: Vec<&str> = data.splitn(4, " ").collect();
    if v.len() < 4 {
        println!("{}", line);
        return;
    }
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
                            output_to_files(values,file_path.clone(), true);
                            fs::remove_file(&file_path).expect("Failed to remove template files");
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
        }
    }
}

pub fn ent_org(){
    let org_dir = "source_files/th/data-original";
    let entries = fs::read_dir(org_dir).unwrap();
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            let path = path.to_str().unwrap().to_string();
            let values: Vec<f32> = ent_compute(&path);
            output_to_files(values, path, false);
        }
    }
}

fn output_to_files(values: Vec<f32>, trace_file: String, encrypt: bool) {

    let res= Res {
        file: trace_file,
        entropy: values[ValueType::_Entropy as usize],
        chisquare: values[ValueType::_ChiSquare as usize],
        mean: values[ValueType::_Mean as usize],
        montecarlo: values[ValueType::_MontecarloPi as usize],
        serial: values[ValueType::_SerialCorrelation as usize],
    };

    let dir = "build/threshold_output";
    if !Path::new(dir).exists() {
        fs::create_dir_all(dir).unwrap();
    }

    let output_path: String;
    if encrypt {
        output_path = unsafe {dir.to_owned()+ "/encrypted_" + &_N.to_string()};
    } else {
        output_path = unsafe {dir.to_owned()+ "/original"};
    }
    if !Path::new(output_path.as_str()).exists() {
        let _result = fs::File::create(output_path.as_str()).unwrap();
    }
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(output_path.as_str())
        .unwrap();

    let buf = serde_json::to_string(&res).unwrap() + "\n";

    if !buf.contains("fspf") &&
        !buf.contains("sgx-musl.conf") &&
        !buf.contains("mmap_min_addr") &&
        !buf.contains("mounts") &&
        !buf.contains("os-release") {
            file.write(buf.as_bytes()).unwrap();
        }
    if buf.contains("fspf") {
        unsafe{
            _N += 1;
        }
    }
}
