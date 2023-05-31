use std::{fs, process::{Command, Output}, path::Path, io::{BufRead, Write, BufReader}};
use serde::{Serialize, Deserialize};
use serde_json;
use encoding_rs_io::{self, DecodeReaderBytesBuilder};
use encoding_rs;

struct Value {
    entropy: f32,
    chi_square: f32,
    mean: f32,
    monte_carlo_pi: f32,
    serial_correlattion: f32
}

#[derive(Deserialize, Serialize, Debug)]
struct Sys {
    syscall: String,
    buf: String,
    ret: String,
}

pub fn analyse(threshold: bool) {
    install_ent();

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
        let data = v["data"].to_string();
        
        let v: Vec<&str> = data.splitn(3, " ").collect();
        let sys = Sys {
            syscall: v[0].to_string(),
            buf: v[2].to_string(),
            ret: v[1].to_string()
        };
        let encrypted = buf_analyse(&sys, threshold);
        if threshold {
            println!("syscall: {}, read length: {}, message: {}", sys.syscall, sys.ret, sys.buf);
        } else {
            if !encrypted {
                println!("The system call: {} is not encrypted, with message: {}.", sys.syscall, sys.buf);
            }
        }
    }
    clean_files();
}

fn buf_analyse(sys: &Sys, threshold: bool) -> bool {
    if sys.buf.len() <= 1 {
        return true;
    } else {
        let mut tmp_file = fs::File::create("files/tmp").unwrap();
        tmp_file.write(sys.buf.as_bytes()).unwrap();

        let result = Command::new("/usr/bin/ent")
            .args(["-t", "files/tmp"])
            .output()
            .unwrap();
        // ------get encheck result----------
        let value = get_chi_square(result);
        if threshold {
            println!("Entropy: {}, Chi Square: {}, Arithmetic Mean: {}, Monte Carlo Value: {}, Serial Correlation Coefficient: {}", 
            value.entropy, value.chi_square, value.mean, value.monte_carlo_pi, value.serial_correlattion);
        } else {
            if value.mean < 100.0 && value.monte_carlo_pi == 4.0 {
                return false;
            }
        }
        return true;
    }
}

fn clean_files() {
    if let Ok(_) = fs::File::open("files/tmp") {
        fs::remove_file("files/tmp").unwrap();
    }
}

fn get_chi_square(result: Output) -> Value {
    let res = String::from_utf8(result.stdout).unwrap();
    // println!("{}", res);
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
    let serial_correlattion = serial_correlation_s.parse::<f32>().unwrap();
    let value = Value {
        entropy,
        chi_square,
        mean,
        monte_carlo_pi,
        serial_correlattion,
    };
    value
}

fn install_ent() {
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
