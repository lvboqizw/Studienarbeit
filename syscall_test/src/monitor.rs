use std::{fs, fs::{File, OpenOptions}, process::{Command, Output}, path::{Path, PathBuf}, 
    io::{BufRead, Write, BufReader, BufWriter}, collections::HashMap, borrow::{Borrow, BorrowMut}};
use serde::{Serialize, Deserialize};
use serde_json;
use encoding_rs_io::{self, DecodeReaderBytesBuilder};
use encoding_rs;

static mut METHOD: &str = "MEAN";
static mut THRESHOLD: f32 = 100.0;
static mut COMPARE: &str = "G";  // G: greater, NE: not equal, L: less

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

pub fn analysis() {
    encryption_analysis();
    
    // let path = Path::new("outfiles");

        // let encrypted = buf_analyse(&sys, threshold);
        // if threshold {
        //     println!("syscall: {}, read length: {}, message: {}", sys.syscall, sys.ret, sys.buf);
        // } else {
        //     if !encrypted {
        //         println!("The system call: {} is not encrypted, with message: {}.", sys.syscall, sys.buf);
        //     }
        // }
    // clean_files();
}

fn encryption_analysis() {
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
                        let value: bool;
                        unsafe{value = evaluate(&result, THRESHOLD, METHOD, COMPARE);}
                        if !value {
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

fn evaluate(result: &Output, threshold: f32, method: &str, compare: &str) -> bool {
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
                    if mean > threshold {
                        return true;
                    }
                },
                "NE" => {
                    if mean != threshold {
                        return true;
                    }
                },
                "L" => {
                    if mean < threshold {
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
                    if entropy > threshold {
                        return true;
                    }
                },
                "NE" => {
                    if entropy != threshold {
                        return true;
                    }
                },
                "L" => {
                    if entropy < threshold {
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
                    if chi_square > threshold {
                        return true;
                    }
                },
                "NE" => {
                    if chi_square != threshold {
                        return true;
                    }
                },
                "L" => {
                    if chi_square < threshold {
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
                    if monte_carlo_pi > threshold {
                        return true;
                    }
                },
                "NE" => {
                    if monte_carlo_pi != threshold {
                        return true;
                    }
                },
                "L" => {
                    if monte_carlo_pi < threshold {
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
                    if serial_correlation > threshold {
                        return true;
                    }
                },
                "NE" => {
                    if serial_correlation != threshold {
                        return true;
                    }
                },
                "L" => {
                    if serial_correlation < threshold {
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

// fn buf_analyse(buf: String, threshold: bool) -> bool {
//     if buf.len() <= 1 {
//         return true;
//     } else {
//         let mut tmp_file = fs::File::create("files/tmp").unwrap();
//         tmp_file.write(buf.as_bytes()).unwrap();

//         let result = Command::new("/usr/bin/ent")
//             .args(["-t", "files/tmp"])
//             .output()
//             .unwrap();
//         // ------get encheck result----------
//         // let value = get_chi_square(result);
//         // if threshold {
//         //     println!("Entropy: {}, Chi Square: {}, Arithmetic Mean: {}, Monte Carlo Value: {}, Serial Correlation Coefficient: {}", 
//         //     value.entropy, value.chi_square, value.mean, value.monte_carlo_pi, value.serial_correlattion);
//         // } else {
//         //     if value.mean < 100.0 && value.monte_carlo_pi == 4.0 {
//         //         return false;
//         //     }
//         // }
//         return true;
//     }
// }




// fn ent_analysis(dir: &Path, encrypt: bool) {
//     let entries = fs::read_dir(dir).unwrap();
    
//     for entry in entries {
//         let entry = entry.unwrap();
//         let result = Command::new("/usr/bin/ent")
//             .args(["-t", entry.path().to_str().unwrap()])
//             .output()
//             .unwrap();
//         let value = get_ent_value(Box::new(entry.path()), &result);
        

//     }
// }

// fn get_ent_value(path: Box<PathBuf>, result: &Output) -> Value {
//     let res = String::from_utf8(result.stdout.clone()).unwrap();

//     let v: Vec<&str> = res.split("\n").collect();
//     let data = v[1].to_string();
//     let v_data: Vec<&str> = data.split(",").collect();

//     let _file_bytes_s = v_data[1].to_string();
//     let entropy_s = v_data[2].to_string();
//     let entropy = entropy_s.parse::<f32>().unwrap();
//     let chi_square_s = v_data[3].to_string();           // 5-10%, 90-95%
//     let chi_square = chi_square_s.parse::<f32>().unwrap();
//     let mean_s = v_data[4].to_string();                 // 127.5 = random
//     let mean = mean_s.parse::<f32>().unwrap();
//     let monte_carlo_pi_s = v_data[5].to_string();       // close to pi, approximation converges very slowly
//     let monte_carlo_pi = monte_carlo_pi_s.parse::<f32>().unwrap();
//     let serial_correlation_s = v_data[6].to_string();   // close to zero -> random
//     let serial_correlation = serial_correlation_s.parse::<f32>().unwrap();
//     let value = Value {
//         path,
//         entropy,
//         chi_square,
//         mean,
//         monte_carlo_pi,
//         serial_correlation,
//     };
//     value
// }