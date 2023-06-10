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

pub fn syscall_analysis() {

    
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