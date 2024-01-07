use std::{path::Path, fs};
use std::process::Command;
use std::thread;
use std::time::Duration;
use structopt::StructOpt;

mod executor;
mod monitor;
mod tracer;
mod threshold;
mod interceptor;

#[derive(StructOpt, Debug)]
enum Com {
    /// Use the system call generator to test the function of test with protected files.
    TestEnc,
    /// Use the system call generator to test the function of test with unporteced files.
    TestOri,
    /// Comparison mode, to find threshold. Should at test mode first.
    Threshold ,
    /// Trace the target application with the given name.
    App {
        /// Use program to trace the target application with the name of application
        #[structopt(long)]
        name: String,
    },
    /// Clean up the generated files in the folder "files".
    Clear,
}

fn main()  {
    // let cmd = Com::from_args();
    // Create files folder to save result
    let dir = "files".to_string();
    if !Path::new(dir.as_str()).exists() {
        fs::create_dir(dir.as_str()).unwrap();
    }
    sudo::escalate_if_needed().expect("Failed to sudo");
    let _ = interceptor::trace("aaa".to_string(), interceptor::TraceMode::Application);
    interceptor::stop_trace();
    // match cmd {
    //     Com::TestEnc => {
    //         println!("Running in test mode.");
            
    //         // Requset sudo permission
    //         sudo::escalate_if_needed().expect("Failed to sudo");
            
    //         // check and install program ent
    //         install_ent(); 

    //         // Copy the corresponding dockerifle
    //         let source_path = "source_files/Dockerfile_test_en";
    //         let path = "generator/Dockerfile";
    //         std::fs::copy(source_path, path).unwrap();

    //         // Start bpftrace and the container which runs the simulate program
    //         tracer::test_trace();
    //         let container_name = executor::run_executor();
    //         /* Check whether the container are finished and stopped */
    //         let mut flag = String::from("true");
    //         while !flag.eq("'false'\n") {
    //             let is_running = Command::new("docker")
    //                 .args(["inspect", "--format", "'{{.State.Running}}'", container_name.as_str()])
    //                 .output()
    //                 .unwrap();
    //             flag = String::from_utf8(is_running.clone().stdout).unwrap();
    //         }
    //         tracer::stop_trace();
            
    //         // Analyse the trace result
    //         monitor::analyse();
    //     },
    //     Com::TestOri => {
    //         println!("Running in test mode.");
            
    //         // // Create files folder to save result
    //         // let dir = "files".to_string();
    //         // if !Path::new(dir.as_str()).exists() {
    //         //     fs::create_dir(dir.as_str()).unwrap();
    //         // }
            
    //         // Requset sudo permission
    //         sudo::escalate_if_needed().expect("Failed to sudo");
            
    //         // check and install program ent
    //         install_ent(); 

    //         // Copy the corresponding dockerifle
    //         let source_path = "source_files/Dockerfile_test_ori";
    //         let path = "generator/Dockerfile";
    //         std::fs::copy(source_path, path).unwrap();

    //         // Start bpftrace and the container which runs the simulate program
    //         tracer::test_trace();
    //         let container_name = executor::run_executor();
    //         /* Check whether the container are finished and stopped */
    //         let mut flag = String::from("true");
    //         while !flag.eq("'false'\n") {
    //             let is_running = Command::new("docker")
    //                 .args(["inspect", "--format", "'{{.State.Running}}'", container_name.as_str()])
    //                 .output()
    //                 .unwrap();
    //             flag = String::from_utf8(is_running.clone().stdout).unwrap();
    //         }
    //         tracer::stop_trace();
            
    //         // Analyse the trace result
    //         monitor::analyse();
    //     },
    //     Com::Threshold => {
    //         sudo::escalate_if_needed().expect("Failed to sudo");
    //         println!("Running at threshold mode, generating the diagram.");
    //         install_ent(); 
    //         let app_name = "threshold".to_string();
    //         tracer::trace(app_name);
    //         thread::sleep(Duration::from_nanos(800));

    //         let source_path = "source_files/Dockerfile_th";
    //         let path = "generator/Dockerfile";
    //         std::fs::copy(source_path, path).unwrap();

    //         let _container_name = executor::run_executor();

    //         let mut pid = Command::new("pgrep")
    //             .args(["threshold"])
    //             .output()
    //             .expect("Fail to get target program pid");
    //         while String::from_utf8_lossy(&pid.stdout).ne(&String::from("")) {
    //             pid = Command::new("pgrep")
    //                 .args(["threshold"])
    //                 .output()
    //                 .expect("Fail to get target program pid");
    //         }
    //         tracer::stop_trace();

    //         threshold::threshold_analysis();
    //     },
    //     Com::Clear => {
    //         sudo::escalate_if_needed().expect("Failed to sudo");
    //         println!("Clear the \"files/\" and \"outfiles/\" folder");
    //         let files = std::path::Path::new("files");
    //         if files.exists() {
    //             std::fs::remove_dir_all(files).unwrap();
    //         }
    //         let outfiles = Path::new("outfiles");
    //         if outfiles.exists() {
    //             fs::remove_dir_all(outfiles).unwrap();
    //         }
    //     }
    //     Com::App { name} => {
    //         sudo::escalate_if_needed().expect("Failed to sudo");
    //         println!("Running at App mode, tracing the target program with name.");
    //         install_ent(); 
    //         tracer::trace(name.clone());
    //         thread::sleep(Duration::from_nanos(800));

    //         let source_path = "source_files/Dockerfile_th";
    //         let path = "generator/Dockerfile";
    //         std::fs::copy(source_path, path).unwrap();

    //         let _container_name = executor::run_executor();

    //         let mut pid = Command::new("pgrep")
    //             .args([&name])
    //             .output()
    //             .expect("Fail to get target program pid");
    //         while String::from_utf8_lossy(&pid.stdout).ne(&String::from("")) {
    //             pid = Command::new("pgrep")
    //                 .args([&name])
    //                 .output()
    //                 .expect("Fail to get target program pid");
    //         }
    //         tracer::stop_trace();

    //         monitor::analyse();
    //     },
    // }
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