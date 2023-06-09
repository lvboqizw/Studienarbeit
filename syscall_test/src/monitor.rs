use std::{fs, fs::{File, OpenOptions}, process::{Command, Output}, path::{Path, PathBuf}, 
    io::{BufRead, Write, BufReader, BufWriter}, collections::HashMap, borrow::{Borrow, BorrowMut}};
use serde::{Serialize, Deserialize};
use serde_json;
use encoding_rs_io::{self, DecodeReaderBytesBuilder};
use encoding_rs;
use plotters::prelude::*;

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

pub fn analyse(threshold: bool) {

    
        // let encrypted = buf_analyse(&sys, threshold);
        // if threshold {
        //     println!("syscall: {}, read length: {}, message: {}", sys.syscall, sys.ret, sys.buf);
        // } else {
        //     if !encrypted {
        //         println!("The system call: {} is not encrypted, with message: {}.", sys.syscall, sys.buf);
        //     }
        // }
    clean_files();
}

pub fn threshold_analysis() {
    syscall_separate();

    let dir_org = Path::new("generator/data-original/test_files");
    let dir_enc = Path::new("outfiles");

    ent_threshold(dir_enc, true);
    ent_threshold(dir_org, false);

    // let mut file_name: Vec<String> = Vec::new();
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

    draw(entropy);

    clean_files()
}

pub fn install_ent() {
    let ent_file = Path::new("/usr/bin/ent");
    if !ent_file.exists() {
        sudo::escalate_if_needed().expect("Failed to sudo");
        let _install_ent = Command::new("sh")
            .current_dir("source_files")
            .arg("install_ent.sh")
            .spawn()
            .unwrap();
    }
}

fn syscall_separate() {
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
            arg2: v[2].to_string(),   // Ret  open-> fd
            arg3: v[3].to_string()    // Path
            
        };
        match sys.syscall.as_str() {
            "open" => {
                if sys.arg3.contains("data/test_files/") {
                    let len = "data/test_files/".len();
                    let file_name = &sys.arg3[len..sys.arg3.len()];
                    if file_name.len() == 0 {
                        continue;
                    }
                    opend_files.insert(sys.arg2.clone(), file_name.to_string());
                }
            },
            "close" => {
                if opend_files.contains_key(&sys.arg1) {
                    opend_files.remove(&sys.arg1);
                }
            },
            _ => {
                if opend_files.contains_key(&sys.arg1) {
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
            },
        }
    }
}

fn ent_threshold(dir: &Path, encrypt: bool) {
    let f_encrypt = Path::new("files/encrypted");
    let f_original = Path::new("files/original");

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
            let mut f = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(f_encrypt)
                .unwrap();
            let _result = f.write(serialized.as_bytes()).unwrap();
        } else {
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

fn draw(data: Vec<(String, f32)>) {

    if data.is_empty() {
        panic!("The data for drawing is empty");
    }

    let labels: Vec<&str> = data.iter().map(|(label, _)| label.as_str()).collect();

    let root_area = BitMapBackend::new("files/entropy.png", (640, 480))
        .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut min = f32::MAX;
    let mut max = f32::MIN;
    for d in &data {
        if d.1 > max {
            max = d.1;
        }
        if d.1 < min {
            min = d.1;
        }
    }

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 30)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Entropy", ("sans-serif", 30))
        .build_cartesian_2d(labels.into_segmented(), 0.0 ..max + 5.0)
        .unwrap();

    ctx
        .configure_mesh()
        .x_desc("Language")
        .draw()
        .unwrap();

    // TODO Draw line or bar
}

fn buf_analyse(buf: String, threshold: bool) -> bool {
    if buf.len() <= 1 {
        return true;
    } else {
        let mut tmp_file = fs::File::create("files/tmp").unwrap();
        tmp_file.write(buf.as_bytes()).unwrap();

        let result = Command::new("/usr/bin/ent")
            .args(["-t", "files/tmp"])
            .output()
            .unwrap();
        // ------get encheck result----------
        // let value = get_chi_square(result);
        // if threshold {
        //     println!("Entropy: {}, Chi Square: {}, Arithmetic Mean: {}, Monte Carlo Value: {}, Serial Correlation Coefficient: {}", 
        //     value.entropy, value.chi_square, value.mean, value.monte_carlo_pi, value.serial_correlattion);
        // } else {
        //     if value.mean < 100.0 && value.monte_carlo_pi == 4.0 {
        //         return false;
        //     }
        // }
        return true;
    }
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

fn get_ent_value(path: Box<PathBuf>, result: &Output) -> Value {
    let res = String::from_utf8(result.stdout.clone()).unwrap();
    println!("{}", res);
    let v: Vec<&str> = res.split("\n").collect();
    let data = v[1].to_string();
    let v_data: Vec<&str> = data.split(",").collect();

    let _file_bytes_s = v_data[1].to_string();
    let entropy_s = v_data[2].to_string();
    let entropy = entropy_s.parse::<f32>().unwrap();
    let chi_square_s = v_data[3].to_string();           // 5-10%, 90-95%
    let chi_square = chi_square_s.parse::<f32>().unwrap();
    let mean_s = v_data[4].to_string();                 // 127.5 = random
    let mean = mean_s.parse::<f32>().unwrap();
    let monte_carlo_pi_s = v_data[5].to_string();       // close to pi, approximation converges very slowly
    let monte_carlo_pi = monte_carlo_pi_s.parse::<f32>().unwrap();
    let serial_correlation_s = v_data[6].to_string();   // close to zero -> random
    let serial_correlation = serial_correlation_s.parse::<f32>().unwrap();
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