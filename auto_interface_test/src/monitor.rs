use std::{fs,  path::{Path, PathBuf}, process::{Command, Output},
    io::{BufRead, Write, BufReader}, collections::HashMap};
use serde::{Serialize, Deserialize};
use serde_json;
use encoding_rs_io::{self, DecodeReaderBytesBuilder};


#[derive(Debug, Deserialize, Serialize)]
struct Value {
    // path: Box<PathBuf>,
    path: String,
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

pub fn analyse() {
    syscall_separate();

    ent_analyse();
    clean_files();
}

fn syscall_separate() {
    let dir = "outfiles".to_string();
    if !Path::new(dir.as_str()).exists() {
        fs::create_dir_all(dir.as_str()).unwrap();
    }
    let mut used_fd: HashMap<String, String> = HashMap::new();

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
            "open" => {
                let count = sys.arg3.chars().filter(|&c| c == '/').count();
                let tmp_filename = dir.clone() + "/" + sys.arg3.replacen("/", "\\", count).as_str();
                used_fd.insert(sys.arg1.clone(), tmp_filename.to_string());
            },
            "accept4" => {
                let count = sys.arg3.chars().filter(|&c| c == '/').count();
                let tmp_filename = dir.clone() + "/accept4_" + sys.arg3.replacen("/", "\\", count).as_str();
                used_fd.insert(sys.arg2.clone(), tmp_filename.to_string());
            },
            "close" => {
                if used_fd.contains_key(&sys.arg1) {
                    used_fd.remove(&sys.arg1);
                }
            },
            _ => {
                if used_fd.contains_key(&sys.arg1) {
                    // let file_path = dir.clone() + "/" +  used_fd.get(&sys.arg1).unwrap().as_str() ;
                    let file_path = used_fd.get(&sys.arg1).unwrap().as_str();
                    if !Path::new(file_path).exists() {
                        let _result = fs::File::create(file_path).unwrap();
                    }
                    let f = fs::OpenOptions::new()
                        .write(true)
                        .append(true)
                        .open(file_path);
                    match f {
                        Ok(mut file) => {
                            file.write(sys.arg3.as_bytes()).unwrap();
                        },
                        Err(err) => {
                            println!("file path: {}, get error: {}", file_path, err);
                        }
                    }
                }
            },
        }
    }
}

fn ent_analyse() {
    let result_path = Path::new("files/result");
    if !result_path.exists() {
        fs::File::create(result_path).unwrap();
    }

    let mut result = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(result_path)
        .unwrap();
    result.write("Unencrypted Files: \n".as_bytes()).unwrap();

    let target_path = Path::new("outfiles");

    let entries = fs::read_dir(target_path).unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let res = Command::new("/usr/bin/ent")
            .args(["-t", entry.path().to_str().unwrap()])
            .output()
            .unwrap();
        let value = get_ent_value(Box::new(entry.path()), &res);

        if value.serial_correlation < 0.43 {
            let tmp: Vec<&str> = value.path.rsplit("/").collect();
            let count = tmp[0].chars().filter(|&c| c == '\\').count();
            let tmp1 = tmp[0].to_string();
            let mut path = tmp1.replacen("\\", "/", count);
            path = path + "\n";
            result.write(path.as_bytes()).unwrap();
            // println!("The file {:?} is not been encrypted.", path);
        }
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

    let path = path.as_path().display().to_string();
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

fn clean_files() {
    let outfiles = Path::new("outfiles");

    if outfiles.exists() {
        fs::remove_dir_all(outfiles).unwrap();
    }
}