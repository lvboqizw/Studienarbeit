use std::{fs::File, io::Write, env};
use std::process::Command;

fn create_trace_file(target: String) {
    let mut trace_file = File::create("files/trace.bt").unwrap();
    let mut filter = "s/comm==\"\"/comm==\"".to_string();
    filter = filter + target.as_str() + "\"/g";
    let write_target = Command::new("sed")
        .args([filter, "source_files/trace_source.bt".to_string()])
        .output()
        .unwrap();
    trace_file.write(String::from_utf8(write_target.stdout).unwrap().as_bytes()).unwrap();
}

pub fn trace(target: String) {
    create_trace_file(target);
    
    env::set_var("BPFTRACE_STRLEN", "200");
    let _output_file = File::create("files/output.json");

    let _tracer = Command::new("bpftrace")
        .args(["-f", "json", "-o", "files/output.json", "files/trace.bt"])
        .spawn()
        .expect("Failed to run bpftrace");
}

pub fn test_trace() {
    env::set_var("BPFTRACE_STRLEN", "200");
    let _output_file = File::create("output.json").unwrap();
    let _trace_status = Command::new("/home/wei/bpftrace/bin/bpftrace")
        .args(["-f", "json", "-o", "files/output.json", "source_files/test_trace.bt"])
        .spawn()
        .expect("Failed to run bpftrace");
}

pub fn stop_trace() {
    // Stop the bpftrace tracer for the next run
    let _kill_bpftrace = Command::new("pkill")
        .arg("/home/wei/bpftrace/bin/bpftrace")
        .spawn()
        .unwrap();
}