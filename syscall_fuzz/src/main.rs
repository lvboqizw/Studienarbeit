use std::fs;
use structopt::StructOpt;

mod executor;
mod monitor;
mod tracer;


#[derive(StructOpt, Debug)]
struct Opt {
    /// Local container fuzzing
    #[structopt(short, long)]
    local: bool,

    /// Fuzz a container in the kubernets
    #[structopt(short, long)]
    kubernetes: bool,
}

fn main()  {
    let opt = Opt::from_args();
    
    if !opt.local && !opt.kubernetes {
        panic!("Please at least setup one option between local and kubernetes");
    }
    sudo::escalate_if_needed().expect("Failed to sudo"); // bpftrace need to run with root permission
    tracer::trace(String::from("generator"));
    // ------------------------------executor-----------------------------
    executor::run_executor();
    fs::remove_file("files/trace.bt").unwrap();

    tracer::stop_trace();
    // -----------------------------monitor------------------------------
    monitor::output_analysis();
    
}