use std::{fs, process::{Command, Output}, path::{Path, PathBuf}, 
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
    arg1: String,       // fd
    arg2: String,       // Ret  open-> fd
    arg3: String,       // Path
}

pub fn analysis(method: String, value: f32, compare: String) {
    // encryption_analysis(method, value, compare);
    // clean_files();
}

fn encryption_analysis(method: String, value: f32, compare: String) {
    let dir = "outfiles".to_string();
    if !Path::new(dir.as_str()).exists() {
        fs::create_dir_all(dir.as_str()).unwrap();
    }
    let mut opend_files: HashMap<String, (String, String)> = HashMap::new();        // HashMap <fd, (original_path, file_path for ent)>

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
            arg2: v[2].to_string(),   // Ret  open-> fd
            arg3: v[3].to_string()    // Path          
        };
        match sys.syscall.as_str() {
            "open" => {
                // if !Path::new((sys.arg3.clone() + "/").as_str()).is_dir() {
                let count = sys.arg3.chars().filter(|&c| c == '/').count();
                let ent_path =dir.clone() + "/" + sys.syscall.as_str() + "_" + sys.arg3.replacen("/", "_", count).as_str();
                opend_files.insert(sys.arg2.clone(), (sys.arg3.clone(), ent_path.clone()));
                // }
            },
            "close" => {
                // Calculate ent and Analysis result
                if opend_files.contains_key(&sys.arg1) {
                    let paths = opend_files.get(&sys.arg1).unwrap();
                    let path = paths.1.as_str();
                    if Path::new(path).exists() {
                        let result = Command::new("/usr/bin/ent")
                            .args(["-t", path])
                            .output()
                            .unwrap();
                        let result = evaluate(&result, &method, value, &compare);
                        if !result {
                            println!("the file {} is not encrypted.", paths.0);
                        }
                    }
                    opend_files.remove(&sys.arg1);
                }
            },
            _ => {
                if opend_files.contains_key(&sys.arg1) {
                    let paths = opend_files.get(&sys.arg1).unwrap();
                    let file_path =paths.1.as_str();
                    if !Path::new(file_path).exists() {
                        let _result = fs::File::create(file_path).unwrap();
                    }
                    let mut file = fs::OpenOptions::new()
                        .write(true)
                        .append(true)
                        .open(file_path)
                        .unwrap();
                    file.write(sys.arg3.as_bytes()).unwrap();
                }
            },
        }
    }
}

fn evaluate(result: &Output, method: &str, value: f32, compare: &str) -> bool {
    let res = String::from_utf8(result.stdout.clone()).unwrap();
    println!("res: {}", res);
    let v: Vec<&str> = res.split("\n").collect();
    let data = v[1].to_string();
    let v_data: Vec<&str> = data.split(",").collect();
    match method {
        "MEAN" => {
            let mean_s = v_data[4].to_string();                 // 127.5 = random
            let mean = mean_s.parse::<f32>().unwrap();
            match compare {
                "G" => {
                    if mean > value {
                        return true;
                    }
                },
                "NE" => {
                    if mean != value {
                        return true;
                    }
                },
                "L" => {
                    if mean < value {
                        return true;
                    }
                },
                _ => {
                    panic!("Wrong COMPARE argument!");
                },
            };
        },
        "ENTROPY" => {
            let entropy_s = v_data[2].to_string();
            let entropy = entropy_s.parse::<f32>().unwrap();    
            match compare {
                "G" => {
                    if entropy > value {
                        return true;
                    }
                },
                "NE" => {
                    if entropy != value {
                        return true;
                    }
                },
                "L" => {
                    if entropy < value {
                        return true;
                    }
                },
                _ => {
                    panic!("Wrong COMPARE argument!");
                },
            };
        },
        "CHI_SQUARE" => {
            let chi_square_s = v_data[3].to_string();           // 5-10%, 90-95%
            let chi_square = chi_square_s.parse::<f32>().unwrap();
            match compare {
                "G" => {
                    if chi_square > value {
                        return true;
                    }
                },
                "NE" => {
                    if chi_square != value {
                        return true;
                    }
                },
                "L" => {
                    if chi_square < value {
                        return true;
                    }
                },
                _ => {
                    panic!("Wrong COMPARE argument!");
                },
            };
        },
        "MONTE_CARLO" => {
            let monte_carlo_pi_s = v_data[5].to_string();       // close to pi, approximation converges very slowly
            let monte_carlo_pi = monte_carlo_pi_s.parse::<f32>().unwrap();
            match compare {
                "G" => {
                    if monte_carlo_pi > value {
                        return true;
                    }
                },
                "NE" => {
                    if monte_carlo_pi != value {
                        return true;
                    }
                },
                "L" => {
                    if monte_carlo_pi < value {
                        return true;
                    }
                },
                _ => {
                    panic!("Wrong COMPARE argument!");
                },
            };
        },
        "SERIAL_CORRELATION" => {
            let serial_correlation_s = v_data[6].to_string();   // close to zero -> random
            let serial_correlation = serial_correlation_s.parse::<f32>().unwrap();
            match compare {
                "G" => {
                    if serial_correlation > value {
                        return true;
                    }
                },
                "NE" => {
                    if serial_correlation != value {
                        return true;
                    }
                },
                "L" => {
                    if serial_correlation < value {
                        return true;
                    }
                },
                _ => {
                    panic!("Wrong COMPARE argument!");
                },
            };
        },
        _ => {
            panic!("Wrong METHOD argument!");
        },
    };
    false
}

fn clean_files() {
    // if let Ok(_) = fs::File::open("files/tmp") {
    //     fs::remove_file("files/tmp").unwrap();
    // }
    let outfiles = Path::new("outfiles");

    if outfiles.exists() {
        fs::remove_dir_all(outfiles).unwrap();
    }
}