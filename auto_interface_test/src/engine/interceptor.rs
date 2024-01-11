// use std::os::unix::prelude::PermissionsExt;
use std::fs::File;
use std::env;
use std::process::{Command, Stdio, Child};
use std::io::{self, BufRead, BufReader, Write};

use super::TraceMode;

pub fn trace(target: String, mode: TraceMode) -> io::Result<Child> {
    if target.len() == 0 && mode.eq(&TraceMode::Application)  {
        panic!("The name of target application is request!");
    }
    
    env::set_var("BPFTRACE_STRLEN", "200");

    let bpf = Command::new("which")
        .arg("bpftrace")
        .output()
        .unwrap();
    let tmp = String::from_utf8(bpf.stdout).unwrap();
    let bpf_path = tmp.trim();

    let res = create_script_file(target);
    let trace_fp = match res {
        Ok(path) => path,
        Err(err) => {
            panic!("Failed to creat trace script file: {}", err);
        },
    };

    match mode {
        TraceMode::Test => {
            let child = Command::new(bpf_path)
                .args(["-f", "json", "source_files/trace_test.bt"])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()?;
            return Ok(child);
        },
        TraceMode::Threshold => {
            let child = Command::new(bpf_path)
                .args(["-f", "json", &trace_fp])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()?;
            return Ok(child);
        },
        TraceMode::Application => {
            let child = Command::new(bpf_path)
                .args(["-f", "json", &trace_fp])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()?;
            return Ok(child);
        }
    }

}

fn create_script_file(target: String) -> io::Result<String>{
    let org_path = "source_files/tmp_trace.bt";
    let org_file = File::open(&org_path)?;

    let output_path = "build/trace.bt";
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

    Ok(output_path.to_string())
}

pub fn stop_trace() {
    // Stop the bpftrace tracer for the next run
    let _kill_bpftrace = Command::new("pkill")
        .arg("bpftrace")
        .spawn()
        .unwrap();
}