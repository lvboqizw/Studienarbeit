use std::{fs::OpenOptions, io::Write, path::Path, fs};
use std::process::Command;
use std::thread;
use std::time::Duration;
use structopt::StructOpt;

mod executor;
mod monitor;
mod tracer;
mod threshold;

#[derive(StructOpt, Debug)]
enum Com {
    /// Use the system call generator to test the function of test
    Test,
    // Test {
    //     /// Algorithm: MEAN, ENTROPY, CHI_SQUARE, MONTE_CARLO, SERIAL_CORRELATION
    //     #[structopt( long, default_value = "Mean")]
    //     alg: String,

    //     /// Threshold value: you can base on the output digram of Threshold mode
    //     #[structopt( long, default_value = "100.0")]
    //     value: f32,

    //     /// What kind of value is random: G: greater than threshold value, NE: not equal, L: less than
    //     #[structopt( long, default_value = "G")]
    //     compare: String,
    // },
    /// Comparison mode, to find threshold. Should at test mode first
    Threshold ,
    /// Trace the target application with the given name
    App {
        /// Use program to trace the target application with the name of application
        #[structopt(long)]
        name: String,
        /// Algorithm: MEAN, ENTROPY, CHI_SQUARE, MONTE_CARLO, SERIAL_CORRELATION
        #[structopt( long, default_value = "Mean")]
        alg: String,

        /// Threshold value: you can base on the output digram of Threshold mode
        #[structopt( long, default_value = "100.0")]
        value: f32,

        /// What kind of value is random: G: greater than threshold value, NE: not equal, L: less than
        #[structopt( long, default_value = "G")]
        compare: String,
    },
    /// Clean up the generated files in the folder "files"
    Clear,
}

fn main()  {
    let cmd = Com::from_args();

    match cmd {
        Com::Test => {
            println!("Running in test mode.");
            let dir = "files".to_string();
            if !Path::new(dir.as_str()).exists() {
                fs::create_dir(dir.as_str()).unwrap();
            }
            
            sudo::escalate_if_needed().expect("Failed to sudo");
            install_ent(); 
            let source_path = "source_files/Dockerfile_test";
            let path = "generator/Dockerfile";
            std::fs::copy(source_path, path).unwrap();
            tracer::test_trace();

            let container_name = executor::run_executor();
            /* Check whether the container are finished and stopped */
            let mut flag = String::from("true");
            while !flag.eq("'false'\n") {
                let is_running = Command::new("docker")
                    .args(["inspect", "--format", "'{{.State.Running}}'", container_name.as_str()])
                    .output()
                    .unwrap();
                flag = String::from_utf8(is_running.clone().stdout).unwrap();
            }
            tracer::stop_trace();

            monitor::analyse();
            // monitor::analysis(alg, value, compare);
        },
        Com::Threshold => {
            println!("Running at threshold mode, generating the diagram.");
            let dir = "files".to_string();
            if !Path::new(dir.as_str()).exists() {
                fs::create_dir(dir.as_str()).unwrap();
            }

            sudo::escalate_if_needed().expect("Failed to sudo");
            install_ent(); 
            let app_name = "threshold".to_string();
            tracer::trace(app_name);
            thread::sleep(Duration::from_nanos(800));

            let source_path = "source_files/Dockerfile_th";
            let path = "generator/Dockerfile";
            std::fs::copy(source_path, path).unwrap();

            let container_name = executor::run_executor();
            /* Check whether the container are finished and stopped */
            let mut flag = String::from("true");
            while !flag.eq("'false'\n") {
                let is_running = Command::new("docker")
                    .args(["inspect", "--format", "'{{.State.Running}}'", container_name.as_str()])
                    .output()
                    .unwrap();
                flag = String::from_utf8(is_running.clone().stdout).unwrap();
            }
            tracer::stop_trace();

            threshold::threshold_analysis();
        },
        Com::Clear => {
            println!("Clear the \"files/\" folder");
            let files = std::path::Path::new("files");
            if files.exists() {
                std::fs::remove_dir_all(files).unwrap();
            }
            let outfiles = Path::new("outfiles");
            if outfiles.exists() {
                fs::remove_dir_all(outfiles).unwrap();
            }
            // std::fs::create_dir(path).unwrap();
        }
        Com::App { name, alg, value, compare } => {
            // println!("Analysing the program {}, with alg: {}, value:{} , compare:{} ", name, alg, value, compare);
        },
    }
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