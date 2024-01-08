// use std::os::unix::prelude::PermissionsExt;
use std::fs::File;
use std::env;
use std::process::{Command, Stdio};
use std::io::{self, BufRead, BufReader, Write, Read};

use crate::engine;

pub enum TraceMode {
    Test,
    Application,
}

fn create_script_file(target: String) -> io::Result<()>{
    let org_path = "source_files/tmp_trace.bt";
    let org_file = File::open(&org_path)?;

    let output_path = "files/trace.bt";
    let mut output_file = File::create(output_path).unwrap();

    let org_filter = "comm==\"";
    let mut tar_filter = "comm==\"".to_string();
    tar_filter = tar_filter + target.as_str(); 

    let reader = BufReader::new(org_file);
    for line in reader.lines() {
        let org_line = line?;

        let modified_line = org_line.replace(org_filter, tar_filter.as_str());
        
        writeln!(&mut output_file, "{}", modified_line)?;
    }

    Ok(())
}

pub fn trace(target: String, mode: TraceMode) -> io::Result<()>{
    
    env::set_var("BPFTRACE_STRLEN", "200");
    // let _output_file = File::create("files/output.json")?;

    let bpf = Command::new("which")
        .arg("bpftrace")
        .output()?;
    let tmp = String::from_utf8(bpf.stdout).unwrap();
    let bpf_path = tmp.trim();

    match mode {
        TraceMode::Test => {
            let _child = Command::new(bpf_path)
                .args(["-f", "json", "-o", "files/output.json", "source_files/test_trace.bt"])
                .spawn()
                .expect("Failed to run bpftrace");
        },
        TraceMode::Application => {
            let _ = create_script_file(target);
            let mut child = Command::new(bpf_path)
                // .args(["-f", "json", "-o", "files/output.json", "files/trace.bt"])
                .args(["-f", "json", "files/trace.bt"])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("Failed to run bpftrace");

                let child_out = BufReader::new(child.stdout.as_mut().unwrap());

                for line in child_out.lines() {
                    if let Ok(line) = line {
                        engine::threshold_analysis(line);
                    }
                }
        }
    }

    Ok(())
}

pub fn stop_trace() {
    // Stop the bpftrace tracer for the next run
    let _kill_bpftrace = Command::new("pkill")
        .arg("bpftrace")
        .spawn()
        .unwrap();
}