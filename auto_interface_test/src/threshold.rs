use std::{fs, fs::{File, OpenOptions}, process::{Command, Output}, path::{Path, PathBuf}, 
    io::{BufRead, Write, BufReader}, collections::HashMap};
use serde::{Serialize, Deserialize};
use serde_json;
use encoding_rs_io::{self, DecodeReaderBytesBuilder};
use encoding_rs;


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

pub fn threshold_analysis() {
    // println!("");
    syscall_separate_th();
    let dir_org = Path::new("generator/data-original/test_files");
    let dir_enc = Path::new("outfiles");

    ent_threshold(dir_enc, true);
    ent_threshold(dir_org, false);

    let mut entropy: Vec<(String, f32)> = Vec::new();
    let mut chi_sq: Vec<(String, f32)> = Vec::new();
    let mut mean: Vec<(String, f32)> = Vec::new();
    let mut monte_carlo: Vec<(String, f32)> = Vec::new();
    let mut serial_correlation: Vec<(String, f32)> = Vec::new();

    let encrypted = Path::new("files/encrypted");
    let file = File::open(encrypted).unwrap();
    let lines = BufReader::new(file).lines();
    for line in lines {
        let value: Value = serde_json::from_str(line.unwrap().as_str()).unwrap();
        let tmp: Vec<&str> = value.path.to_str().unwrap().rsplit("/").collect();
        // file_name.push(tmp[0].to_string());
        entropy.push((tmp[0].to_string(),value.entropy));
        chi_sq.push((tmp[0].to_string(),value.chi_square));
        mean.push((tmp[0].to_string(),value.mean));
        monte_carlo.push((tmp[0].to_string(),value.monte_carlo_pi));
        serial_correlation.push((tmp[0].to_string(),value.serial_correlation));
    }


    let mut entropy_org: Vec<(String, f32)> = Vec::new();
    let mut chi_sq_org: Vec<(String, f32)> = Vec::new();
    let mut mean_org: Vec<(String, f32)> = Vec::new();
    let mut monte_carlo_org: Vec<(String, f32)> = Vec::new();
    let mut serial_correlation_org: Vec<(String, f32)> = Vec::new();

    let original = Path::new("files/original");
    let file = File::open(original).unwrap();
    let lines = BufReader::new(file).lines();
    for line in lines {
        let value: Value = serde_json::from_str(line.unwrap().as_str()).unwrap();
        let tmp: Vec<&str> = value.path.to_str().unwrap().rsplit("/").collect();
        // file_name.push(tmp[0].to_string());
        entropy_org.push((tmp[0].to_string(),value.entropy));
        chi_sq_org.push((tmp[0].to_string(),value.chi_square));
        mean_org.push((tmp[0].to_string(),value.mean));
        monte_carlo_org.push((tmp[0].to_string(),value.monte_carlo_pi));
        serial_correlation_org.push((tmp[0].to_string(),value.serial_correlation));
    }

    clean_files();
}

/// Process bpftrace output files, consolidate Syscalls by fd into temporary files
fn syscall_separate_th() {
    let dir = "outfiles".to_string();
    if !Path::new(dir.as_str()).exists() {
        fs::create_dir_all(dir.as_str()).unwrap();
    }
    let mut opend_files: HashMap<String, String> = HashMap::new();

    let file = fs::File::open("files/output.json").unwrap();
    let mut lines = BufReader::new(
        DecodeReaderBytesBuilder::new()
        .encoding(Some(encoding_rs::UTF_8))
        .build(file))
        .lines();
    lines.next();


    for line in lines {
        let l = line.unwrap();
        let v: serde_json::Value = serde_json::from_str(l.as_str()).unwrap();
        let mut data = v["data"].to_string();
        data.remove(0);
        data.remove(data.len() - 1);
        
        let v: Vec<&str> = data.splitn(4, " ").collect();
        let sys = Sys {
            syscall: v[0].to_string(),
            arg1: v[1].to_string(),   // fd
            arg2: v[2].to_string(),   // Ret  open-> _
            arg3: v[3].to_string()    // Data/Path
            
        };
        match sys.syscall.as_str() {
            "open"|"openat"  => {
                if sys.arg3.contains("data/test_files/") {
                    let len = "data/test_files/".len();
                    let file_name = &sys.arg3[len..sys.arg3.len()];
                    if file_name.len() == 0 {
                        continue;
                    }
                    opend_files.insert(sys.arg1.clone(), file_name.to_string());
                }
            },
            "close" => {
                if opend_files.contains_key(&sys.arg1) {
                    opend_files.remove(&sys.arg1);
                }
            },
            _ => {
                let buf_len = sys.arg2.parse::<u32>().unwrap();
                if opend_files.contains_key(&sys.arg1) {
                    if buf_len != 0 {
                        let file_path = dir.clone() + "/" +  opend_files.get(&sys.arg1).unwrap().as_str() ;
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
            },
        }
    }
}

fn ent_threshold(dir: &Path, encrypt: bool) {

    let entries = fs::read_dir(dir).unwrap();
    
    for entry in entries {
        let entry = entry.unwrap();
        let result = Command::new("/usr/bin/ent")
            .args(["-t", entry.path().to_str().unwrap()])
            .output()
            .unwrap();
        let value = get_ent_value(Box::new(entry.path()), &result);
        
        let mut serialized = serde_json::to_string(&value).unwrap();
        serialized = serialized + "\n";
        if encrypt {
            let f_encrypt = Path::new("files/encrypted");
            let mut f = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(f_encrypt)
                .unwrap();
            let _result = f.write(serialized.as_bytes()).unwrap();
        } else {
            let f_original = Path::new("files/original");
            let mut f = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(f_original)
                .unwrap();
            let _result = f.write(serialized.as_bytes()).unwrap();
        }
    }
}

fn clean_files() {
    let outfiles = Path::new("outfiles");

    if outfiles.exists() {
        fs::remove_dir_all(outfiles).unwrap();
    }
}

fn get_ent_value(path: Box<PathBuf>, result: &Output) -> Value {
    let res = String::from_utf8(result.stdout.clone()).unwrap();
    let v: Vec<&str> = res.split("\n").collect();
    let data = v[1].to_string();
    let v_data: Vec<&str> = data.split(",").collect();

    let _file_bytes_s = v_data[1].to_string();
    let entropy_s = v_data[2].to_string();
    let mut entropy = entropy_s.parse::<f32>().unwrap();
    if entropy_s.contains("nan") {
        entropy = 0.0;
    }
    let chi_square_s = v_data[3].to_string();           // 5-10%, 90-95%
    let mut chi_square = chi_square_s.parse::<f32>().unwrap();
    if chi_square_s.contains("nan") {
        chi_square = 0.0;
    }
    let mean_s = v_data[4].to_string();                 // 127.5 = random
    let mut mean = mean_s.parse::<f32>().unwrap();
    if mean_s.contains("nan") {
        mean = 0.0;
    }
    let monte_carlo_pi_s = v_data[5].to_string();       // close to pi, approximation converges very slowly
    let mut monte_carlo_pi = monte_carlo_pi_s.parse::<f32>().unwrap();
    if monte_carlo_pi_s.contains("nan") {
        monte_carlo_pi = 0.0;
    }
    let serial_correlation_s = v_data[6].to_string();   // close to zero -> random
    let mut serial_correlation = serial_correlation_s.parse::<f32>().unwrap();
    if serial_correlation_s.contains("nan") {
        serial_correlation = 0.0;
    }
    let value = Value {
        path,
        entropy,
        chi_square,
        mean,
        monte_carlo_pi,
        serial_correlation,
    };
    value
}