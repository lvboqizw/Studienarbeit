use std::{fs::OpenOptions, io::Write};
use std::process::Command;
use structopt::StructOpt;

mod executor;
mod monitor;
mod tracer;


#[derive(StructOpt, Debug)]
struct Opt {
    /// Use the system call generator to test the function of test
    #[structopt(short = "t", long = "test")]
    test: bool,

    /// Comparison mode, to find threshold. Should at test mode first
    #[structopt(short = "h", long = "threshold")]
    threshold: bool,

    /// Give the name of the target application
    #[structopt(short, long)]
    app: Option<String>,
}

fn main()  {
    // bpftrace need to run with root permission
    sudo::escalate_if_needed().expect("Failed to sudo"); 
    
    let app_name: String;
    let opt = Opt::from_args();
    if opt.test {
        let source_path = "source_files/Dockerfile";
        let path = "generator/Dockerfile";
        std::fs::copy(source_path, path).unwrap();
        let mut fs = OpenOptions::new()
                .write(true)
                .append(true)
                .open(path).unwrap();
        if opt.threshold {
            let execution = "\n CMD [\"sh\", \"/operation/threshold.sh\"]";
            fs.write_all(execution.as_bytes()).unwrap();
            app_name = "threshold".to_string();
            tracer::trace(app_name);
        } else {
            let execution = "\n CMD [\"sh\", \"/operation/run_program.sh\"]";
            fs.write_all(execution.as_bytes()).unwrap();
            tracer::test_trace();
        }

        println!("bpftrace started, waiting for the container");
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
        monitor::analyse(opt.threshold);
    } else if opt.app != None {
        println!("test application {}", opt.app.unwrap());
    } else {
        panic!("Forget to give the test type");
    }

}