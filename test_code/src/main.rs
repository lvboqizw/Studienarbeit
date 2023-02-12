#![allow(unused)]

use std::{thread, time, path::{PathBuf}, fs};
use std::ffi::{CStr, CString};
use std::process::Command;
use arbitrary::Arbitrary;
use rust_syzlang::{FuzzingExecutionContext, TypeAwareResource};
use rand_chacha;
use rand_core::{RngCore, SeedableRng};
use nix::sys::signal;


mod executor;
mod monitor;
mod tracer;


fn main()  {
    
    sudo::escalate_if_needed().expect("Failed to sudo"); // bpftrace need to run with root permission
    tracer::trace(String::from("generator"));
    // ------------------------------executor-----------------------------
    executor::run_executor();
    fs::remove_file("files/trace.bt");
    // -----------------------------monitor------------------------------
    monitor::output_analysis();
    
}