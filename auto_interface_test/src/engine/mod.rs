use std::fs;
use std::path::Path;
use std::process::Command;
use std::io::{BufRead, BufReader};

mod threshold;
mod computer;
mod interceptor;
mod trigger;
mod application;

#[derive(Debug, PartialEq, Clone, Copy)]
enum ValueType {
    _FileBytes           = 0,
    _Entropy             = 1,
    _ChiSquare           = 2,
    _Mean                = 3,
    _MontecarloPi        = 4,
    _SerialCorrelation   = 5,
    _LAST_               = 6
} 

#[derive(Clone, PartialEq)]
pub enum TraceMode {
    Test,
    Threshold,
    Application,
}

pub fn install_ent() {
    if !Path::new("build/ent/ent").exists() {

        let source_path = "source_files/install_ent.sh";
        let destination_path = "build/install_ent.sh";

        fs::copy(source_path, destination_path).unwrap();

        let exit_status = Command::new("sh")
            .current_dir("build")
            .arg("install_ent.sh")
            .status().unwrap();
        assert!(exit_status.success());
    }
}

pub fn trigger(mode: TraceMode) {
    let image_name = String::from("generator");
    trigger::run_trigger(image_name, mode).unwrap();
}

pub fn exec(target: String, mode: TraceMode) {
    let res = interceptor::trace(target, mode.clone());
    let child_out = match res {
        Ok(mut child) => {
            let stdout = child.stdout.take().unwrap();
            let reader = BufReader::new(stdout);
            reader
        },
        Err(err) => {
            panic!("Failed to trace target: {}", err);
        },
    };

    if mode == TraceMode::Application || 
        mode == TraceMode::Test {
            println!("Might not encrypted files: \n");
        }

    let mut once = false;
    for line in child_out.lines() {
        if let Ok(line) = line {
            match mode {
                TraceMode::Test => {
                    application::analysis(line);
                },
                TraceMode::Threshold => {
                    if !once {
                        once = true;
                        threshold::ent_org();
                    }
                    threshold::threshold_analysis(line);
                },
                TraceMode::Application => {
                    application::analysis(line);
                },
            }   
        }
    }
}