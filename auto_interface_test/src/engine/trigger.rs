use std::process::ExitStatus;
use std::{process::Command, io};

use super::TraceMode;

fn create_container(image_name: & String) {
    let status = Command::new("docker")
                            .args(["run", 
                                    "-d", 
                                    "--rm", 
                                    "--device=/dev/sgx_enclave", 
                                    "--privileged",
                                    "--network=host",
                                    &image_name])
                            .status()
                            .unwrap();

    if !status.success() {
        panic!("The program panic with error {:?} by creating image", status);
    }
}

/// Main function to run the functions in the modul. Use ptrace to traces the system calls of the child process, which is created 
/// to run the container
pub fn run_trigger(image_name: String, mode: TraceMode) -> io::Result<()>{

    // create docker image with specific setting
    let status:ExitStatus = match mode {
        TraceMode::Test => {
            Command::new("docker")
            .current_dir("source_files/test")
            .args(["build", "-t", &image_name, "."])
            .status()
            .unwrap()
        },
        TraceMode::Threshold => {
            Command::new("docker")
            .current_dir("source_files/th")
            .args(["build", "-t", &image_name, "."])
            .status()
            .unwrap()
        },
        TraceMode::Application => {
            panic!("Application shold not reach here");
        },
    };

    if !status.success() {
        panic!("The program panic with error {:?} by creating image", status);
    }
    // create the container command with base on the created image file, to run the traced traget program
    create_container(&image_name);
    Ok(())
}