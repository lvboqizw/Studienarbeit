use std::{fs, process::{Command, Output}, path::Path, io::{BufRead, Write, BufReader}, thread};
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
}

pub fn analyse() {
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
        
        let v: Vec<&str> = data.splitn(2, " ").collect();
        let sys = Sys {
            syscall: v[0].to_string(),
            buf: v[1].to_string()
        };
        let encrypted = buf_analyse(&sys);
        if !encrypted {
            println!("The system call {} with buf{} is not encrypted.", sys.syscall, sys.buf);
        }
    }
    clean_files();
}

fn buf_analyse(sys: &Sys) -> bool {
    let random = 127.5;
    if sys.buf.len() < 10 {
        return true;
    } else {
        let mut tmp_file = fs::File::create("files/tmp").unwrap();
        // println!("{:?}", sys.buf);
        tmp_file.write(sys.buf.as_bytes()).unwrap();

        let result = Command::new("/usr/bin/ent")
            .args(["-t", "files/tmp"])
            .output()
            .unwrap();
        // ------get encheck result----------
        let value = get_chi_square(result);
        if f32::abs(random - value.mean) >= 15.0 {
            return false;
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
    println!("bytes: {}, entropy: {}, chi_square: {}, mean_s: {}, monte_carlo_pi_s: {}, serial: {}", _file_bytes_s, entropy_s, chi_square_s, mean_s, monte_carlo_pi_s, serial_correlation_s);
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

pub fn output_analysis() {
    install_ent();

    let handle = thread::spawn(rewrite_file);
    let _result = handle.join().unwrap();

    let file = fs::File::open("files/output_1.json").expect("Failed to open output_1");
    let v: serde_json::Value = serde_json::from_reader(file).expect("Failed at serde read");
    
    let mut array = v.as_array().unwrap().clone();
    array.remove(0);                        // remove the fisrt line which do not containe the system call information 
    let mut syscalls_and_args: Vec<String> = Vec::new();
    for obj in array {
        let tmp = obj["data"].to_string();
        syscalls_and_args.push(tmp);
    }
    let mut collection: Vec<(String, String)> = Vec::new();
    for sys in syscalls_and_args {
        let mut splitter = sys.splitn(2, ' ');
        let first = splitter.next().unwrap();
        let second = splitter.next().unwrap();
        collection.push((first.to_string(), second.to_string()));
    }
    analyse_buf(collection);
    // fs::File::remove_file("files/output_1.json");
}

fn rewrite_file() {
    let file = fs::File::open("files/output.json").unwrap();
    let mut lines = BufReader::new(
        DecodeReaderBytesBuilder::new()
        .encoding(Some(encoding_rs::UTF_8))
        .build(file)
    ).lines();

    let mut fixed_file = fs::File::create("files/output_1.json").expect("Failed at create output_1.json");
    fixed_file.write("[\n".as_bytes()).unwrap();
    let mut line = lines.next().unwrap().unwrap();
    for value in lines {
        line.push(',');
        line.push('\n');
        println!("{}",line);
        fixed_file.write(line.as_bytes()).unwrap();
        line = value.unwrap();
    }
    line.push('\n');
    fixed_file.write(line.as_bytes()).unwrap();
    fixed_file.write("]\n".as_bytes()).unwrap();
    // fs::remove_file("files/output.json").unwrap();
}

fn analyse_buf(collection: Vec<(String, String)>) {
    let random = 127.5;
    let mut encrypt_file = fs::File::create("files/encrypt.txt").unwrap();
    encrypt_file.write("The unencrypted system call and its buffer context: \n".as_bytes()).unwrap();
        
    for sample in collection {
        let mut tmp_file = fs::File::create("files/tmp").unwrap();
        println!("{}",sample.1);
        tmp_file.write(sample.1.as_bytes()).unwrap();
        
        let result = Command::new("/usr/bin/ent")
            .current_dir("files")
            .args(["-t", "tmp"])
            .output()
            .unwrap();
        
        // ------get encheck result----------
        let value = get_chi_square(result);
        if f32::abs(random - value.mean) >= 7.5 {
            let mut context = String::new();
            context.push_str(sample.0.as_str());
            context.push_str("  ");
            context.push_str(sample.1.as_str());
            context.push_str("\n");
            encrypt_file.write(context.as_bytes()).unwrap();    
        }
    }
    let tmp = Path::new("files/tmp");
    if tmp.exists() {
        fs::remove_file(tmp).unwrap();
    }
}



