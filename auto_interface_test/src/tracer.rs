use std::os::unix::prelude::PermissionsExt;
use std::{fs, fs::File, fs::Permissions, io::Write, env};

use std::process::Command;

fn create_trace_file(target: String) {
    let path = "files/trace.bt";
    let mut trace_file = File::create(path).unwrap();
    let mut filter = "s/comm==\"\"/comm==\"".to_string();
    filter = filter + target.as_str() + "\"/g";
    let write_target = Command::new("sed")
        .args([filter, "source_files/trace_source.bt".to_string()])
        .output()
        .unwrap();
    trace_file.write(String::from_utf8(write_target.stdout).unwrap().as_bytes()).unwrap();

    let permission = Permissions::from_mode(0x777);
    fs::set_permissions(path, permission).unwrap();
}

pub fn trace(target: String) {
    create_trace_file(target);
    env::set_var("BPFTRACE_STRLEN", "200");
    let _output_file = File::create("files/output.json").unwrap();

    let _tracer = Command::new("/home/wei/bpftrace/bin/bpftrace")
        .args(["-f", "json", "-o", "files/output.json", "files/trace.bt"])
        .spawn()
        .expect("Failed to run bpftrace");
}

pub fn test_trace() {
    env::set_var("BPFTRACE_STRLEN", "200");
    let _output_file = File::create("files/output.json").unwrap();
    let _trace_status = Command::new("/home/wei/bpftrace/bin/bpftrace")
        .args(["-f", "json", "-o", "files/output.json", "source_files/test_trace.bt"])
        .spawn()
        .expect("Failed to run bpftrace");
}

pub fn stop_trace() {
    // Stop the bpftrace tracer for the next run
    let _kill_bpftrace = Command::new("pkill")
        .arg("bpftrace")
        .spawn()
        .unwrap();
}