// use std::os::unix::prelude::PermissionsExt;
use std::fs::File;
use std::env;
use std::process::{Command, Stdio, Child};
use std::io::{self, BufRead, BufReader, Write};

use super::TraceMode;


fn create_script_file(target: String) -> io::Result<()>{
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

    Ok(())
}

pub fn trace(target: String, mode: TraceMode) -> Child{
    
    env::set_var("BPFTRACE_STRLEN", "200");

    let bpf = Command::new("which")
        .arg("bpftrace")
        .output()
        .unwrap();
    let tmp = String::from_utf8(bpf.stdout).unwrap();
    let bpf_path = tmp.trim();

    match mode {
        TraceMode::Test => {
            let child = Command::new(bpf_path)
                .args(["-f", "json", "-o", "build/output.json", "source_files/test_trace.bt"])
                .spawn()
                .expect("Failed to run bpftrace");
            return child;
        },
        TraceMode::Application | TraceMode::Threshold => {
            let _ = create_script_file(target);
            let child = Command::new(bpf_path)
                .args(["-f", "json", "build/trace.bt"])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("Failed to run bpftrace");
            return child;
        }
    }

}

pub fn stop_trace() {
    // Stop the bpftrace tracer for the next run
    let _kill_bpftrace = Command::new("pkill")
        .arg("bpftrace")
        .spawn()
        .unwrap();
}