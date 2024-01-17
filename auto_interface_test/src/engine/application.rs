use std::fs; 
use std::sync::Mutex;
use std::path::Path;
use std::collections::HashMap;
use std::io::Write;
use serde_json;
use lazy_static::lazy_static;

use super::computer::ent_compute;
use super::ValueType;

static THRESHOLD: f32 = 3.25;
static METHOD: ValueType = ValueType::_MontecarloPi;

#[derive(Debug)]
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

pub fn analysis(line: String) {
    if line.contains("probes") {
        return;
    }

    let line_sj: serde_json::Value = serde_json::from_str(line.as_str()).unwrap();
    let mut data = line_sj["data"].to_string();
    data.remove(0);                 // remove \" before the line
    data.remove(data.len() - 1);    // remove \" at the end of the line
    
    let v: Vec<&str> = data.splitn(4, " ").collect();
    if v.len() < 4 {
        println!("Exception: {:?}", line);
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
                let true_p = file_name.replace("/", "\\");
                let file_path = dir.clone() + "/" + &true_p;
                // println!("true_p: {:?}, file_path:{:?}", true_p, file_path);
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
                            judge(values,file_path);
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
                    let file_path = tmp.get(&sys.arg1).unwrap().to_owned();
    
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

fn judge(values: Vec<f32>, trace_file: String) {
    // println!("values: {:?}\n trace_file: {}\n", values, trace_file);
    let v: Vec<&str> = trace_file.split("/").collect();
    let tmp = v[v.len() - 1].to_string();
    let _true_p = tmp.replace("\\", "/");

    let _res = values[METHOD as usize];
    // println!("res: {}", _res);
    if _res < (THRESHOLD - 0.05) || _res > (THRESHOLD + 0.05) {
        println!("{}: {}", _true_p, _res);
    }
}