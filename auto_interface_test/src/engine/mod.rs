use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;

mod threshold;
mod computer;

enum ValueType {
    Entropy           = 0,
    ChiSquare         = 1,
    Mean              = 2,
    MontecarloPi      = 3,
    SerialCorrelation = 4,
    _LAST_            = 5
}

pub fn install_ent() {
    if !Path::new("build/ent").exists() {
        // fs::create_dir("build/ent").expect("Unable to create build folder");

        let source_path = "source_files/install_ent.sh";
        let destination_path = "build/install_ent.sh";

        fs::copy(source_path, destination_path).unwrap();

        let exit_status = Command::new("sh")
            .current_dir("build")
            .arg("install_ent.sh")
            .status().unwrap();
        assert!(exit_status.success());
    }
}

pub fn threshold_analysis(line: String) {

    threshold::threshold_analysis(line);
}