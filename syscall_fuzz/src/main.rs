use std::process::Command;
use structopt::StructOpt;

mod executor;
mod monitor;
mod tracer;


#[derive(StructOpt, Debug)]
struct Opt {
    /// Use the system call generator to test the function of fuzzer
    #[structopt(short, long)]
    test: bool,

    /// Fuzz a container in the kubernets
    #[structopt(short, long)]
    app: bool,
}

fn main()  {
    // bpftrace need to run with root permission
    sudo::escalate_if_needed().expect("Failed to sudo"); 
    
    let opt = Opt::from_args();
    if opt.test {
        tracer::test_trace();
        println!("bpftrace started, waiting for the container");
        let container_name = executor::run_executor();
        // Check whether the container are finished and stopped
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
    } else {
        panic!("Forget to give the fuzz type");
    }
    
}