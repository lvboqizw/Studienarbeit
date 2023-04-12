use std::{fs, io::Write, process::{Command, Output, Child}, env};

fn create_image(name: &String) {
    let status = Command::new("docker")
        .current_dir("./generator")
        .args(["build", "-t", name.as_str(), "."])
        .status()
        .unwrap();
    if !status.success() {
        panic!("The program panic with error {:?} by creating image", status);
    }
}

/// Generate a container with file
fn create_container(image_name: & String) -> String{
    let container_name = "container_gen";
    let child = Command::new("docker")
        .args(["run", "-d", 
            "--rm", 
            "--device=/dev/isgx", 
            "--privileged", 
            // "--network=host",
            "--name", &container_name,
            image_name.as_str()])
        .spawn()
        .unwrap();
    container_name.to_string()
}

/// Main function to run the functions in the modul. Use ptrace to traces the system calls of the child process, which is created 
/// to run the container
pub fn run_executor() -> String{
    let image_name = String::from("generator");

    // create docker image with specific setting
    create_image(& image_name);
    // create the container command with base on the created image file, to run the traced traget program
    create_container(& image_name)
}
