use std::{fs::OpenOptions, io::Write, path::Path};
use std::process::Command;
use structopt::StructOpt;

mod executor;
mod monitor;
mod tracer;
mod threshold;


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

    /// Clean up the generated files in the folder "files"
    #[structopt(long = "clear")]
    clear: bool,
}

fn main()  {
    // bpftrace need to run with root permission
    sudo::escalate_if_needed().expect("Failed to sudo");
    install_ent(); 
    
    let app_name: String;
    let opt = Opt::from_args();

    if !opt.app.is_none() {                                         // Trace application
        println!("trace target program: {}", opt.app.as_ref().unwrap());
        return;
    } else if opt.test {                                            // Test
        println!("Run test to verify the function");
        let source_path = "source_files/Dockerfile";
        let path = "generator/Dockerfile";
        std::fs::copy(source_path, path).unwrap();
        let mut fs = OpenOptions::new()
                .write(true)
                .append(true)
                .open(path).unwrap();
        let execution = "\n CMD [\"sh\", \"/operation/run_program.sh\"]";
        fs.write_all(execution.as_bytes()).unwrap();
        tracer::test_trace();
    } else if opt.threshold {                                       // threshold
        println!("Run test to find a fit threshold");
        let source_path = "source_files/Dockerfile";
        let path = "generator/Dockerfile";
        std::fs::copy(source_path, path).unwrap();
        let mut fs = OpenOptions::new()
                .write(true)
                .append(true)
                .open(path).unwrap();
        let execution = "\n CMD [\"sh\", \"/operation/threshold.sh\"]";
        fs.write_all(execution.as_bytes()).unwrap();
        app_name = "threshold".to_string();
        tracer::trace(app_name);
    } else if opt.clear {
        let path = std::path::Path::new("files");
        if path.exists() {
            std::fs::remove_dir_all(path).unwrap();
        }
        std::fs::create_dir(path).unwrap();
        return;
    } else {
        println!("Require a argument, use --help to get information");
        return;
    }

    if opt.test || opt.threshold {
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
    }
    if opt.test {
        println!("Analyse test");
    } else if opt.threshold {
        println!("Analyse threshold");
        threshold::threshold_analysis();
    } else {
        println!("Analyse app tracing");
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