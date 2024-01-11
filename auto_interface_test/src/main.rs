use std::{path::Path, fs};

use std::time::Duration;
use structopt::StructOpt;

mod executor;
mod monitor;
mod tracer;
mod threshold;
mod engine;

use engine::TraceMode;

#[derive(StructOpt, Debug)]
enum Com {
    /// Test the functionality of the program
    Test,
    /// Generate data for finding thresholds
    Threshold ,
    /// Trace the target program with the parameter target program name
    App {
        #[structopt(long)]
        name: String,
    }
}

fn main()  {
    let cmd = Com::from_args();
    // Create files folder to save result
    let dir = "build";
    fs::create_dir_all(dir).unwrap();

    engine::install_ent();

    sudo::escalate_if_needed().expect("Failed to sudo");
    // let _ = engine::intercpet("a".to_string(), TraceMode::Threshold);
    // interceptor::stop_trace();

    match cmd {
        Com::Test => {
            // engine::exec("".to_string(), TraceMode::Test);
        },
        Com::Threshold => {
            engine::exec("a".to_string(), TraceMode::Threshold);
        },
        Com::App { name } => {
            engine::exec(name, TraceMode::Threshold);
        }
    }
}