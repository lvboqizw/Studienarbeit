#![feature(file_create_new)]

use std::{fs, process::{Command, Output}, path::Path, io::{BufRead, Write, BufReader}};
use serde_json;
use encoding_rs_io::{self, DecodeReaderBytesBuilder};
use encoding_rs;

pub fn output_analysis() {
    rewrite_file();
    install_ent();

    let mut file = fs::File::open("files/output_1.json").expect("Failed to open output_1");
    let v: serde_json::Value = serde_json::from_reader(file).expect("Failed at serde read");
    
    let mut array = v.as_array().unwrap().clone();
    array.remove(0);                        // remove the fisrt line which do not containe the system call information 
    let mut syscalls_and_args: Vec<String> = Vec::new();
    for obj in array {
        let tmp = obj["data"].to_string();
        syscalls_and_args.push(tmp);
    }

    let mut collection: Vec<(String, String)> = Vec::new();
    for mut sys in syscalls_and_args {
        let mut splitter = sys.splitn(2, ' ');
        let first = splitter.next().unwrap();
        let second = splitter.next().unwrap();
        collection.push((first.to_string(), second.to_string()));
    }
    analyse_buf(collection);
    // fs::File::remove_file("files/output_1.json");
}

fn rewrite_file() {
    if let Ok(file) = fs::File::open("files/output.json") {
        let mut lines = BufReader::new(
            DecodeReaderBytesBuilder::new()
            .encoding(Some(encoding_rs::UTF_8))
            .build(file)
        ).lines();

        let mut fixed_file = fs::File::create("files/output_1.json").expect("Failed at create output_1.json");
        fixed_file.write("[\n".as_bytes());
        let mut line = lines.next().unwrap().unwrap();
        for mut value in lines {
            line.push(',');
            line.push('\n');
            fixed_file.write(line.as_bytes());
            line = value.unwrap();
        }
        line.push('\n');
        fixed_file.write(line.as_bytes());
        fixed_file.write("]\n".as_bytes());
    } else {
        println!("Failed to open output file");
    }
    fs::remove_file("files/output.json");
}

fn analyse_buf(collection: Vec<(String, String)>) {
    let random = 127.5;
    let mut warning_flag = false;
    let mut encrypt_file = fs::File::create("files/encrypt.txt").unwrap();
    encrypt_file.write("The unencrypted system call and its buffer context: \n".as_bytes());
        
    for sample in collection {
        let mut tmp_file = fs::File::create("files/tmp").unwrap();
        tmp_file.write(sample.1.as_bytes()).unwrap();
        
        let result = Command::new("ent")
            .current_dir("files")
            .args(["-t", "tmp"])
            .output()
            .unwrap();
        
        // ------get encheck result----------
        let mean = get_chi_square(result);
        if f32::abs(random - mean) >= 7.5 {
            if !warning_flag {
                println!{"The tested program is not save!"};
                warning_flag = true;
            }
            let mut context = String::new();
            context.push_str(sample.0.as_str());
            context.push_str("  ");
            context.push_str(sample.1.as_str());
            context.push_str("\n");
            println!{"{}", context};
            encrypt_file.write(context.as_bytes());    
        }
    }
    if !warning_flag {
        println!{"The buffer of system calls of the target program are encrypted."};
    }
    let tmp = Path::new("files/tmp");
    if tmp.exists() {
        fs::remove_file(tmp);
    }
}

fn get_chi_square(result: Output) -> f32{
    let mut res = String::from_utf8(result.stdout).unwrap();
    let v: Vec<&str> = res.split("\n").collect();
    let mut data = v[1].to_string();
    let v_data: Vec<&str> = data.split(",").collect();

    let file_bytes_s = v_data[1].to_string();
    let entropy_s = v_data[2].to_string();
    let chi_square_s = v_data[3].to_string();           // 5-10%, 90-95%
    let mean_s = v_data[4].to_string();                 // 127.5 = random
    let mean = mean_s.parse::<f32>().unwrap();
    let monte_carlo_pi_s = v_data[5].to_string();       // close to pi, approximation converges very slowly
    let serial_correlation_s = v_data[6].to_string();   // close to zero -> random
    mean
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