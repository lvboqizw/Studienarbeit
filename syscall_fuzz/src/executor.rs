use std::{fs, io::Write, process::{Command, Output}, env};

/// Create the dockerfile with the necessary environment arguments, return the image id as String
pub fn create_image() -> String{
    let mut dockerfile = fs::File::create("docker/Dockerfile").expect("failed to create/open Dockerfile");
    dockerfile.write("FROM registry.scontain.com/sconecuratedimages/crosscompilers:ubuntu\n".as_bytes()).unwrap();
    dockerfile.write("CMD [\"sh\", \"execution.sh\"]\n".as_bytes()).expect("Failed to write to Dockerfile");
    let image = Command::new("docker")
        .args(["build", "-t", "scone:target", "."])
        .current_dir("docker")
        .output()
        .expect("Failed to build the docker image");
    get_image_id(image)
}

/// Get generated image's id throught output
fn get_image_id(image: Output) -> String{
    let output = String::from_utf8(image.stdout)
        .expect("fn: get_image_ID. Failed to get String from Output");
    let sentences: Vec<&str> = output.split("\n").collect();
    let sen_with_id = sentences[sentences.len() - 3];
    let words: Vec<&str> = sen_with_id.split(" ").collect();
    let image_id = String::from(words[words.len() - 1]);
    image_id
}

/// Generate a container with file
fn create_container(image_id: String) -> Command{
    let cur_path = String::from(env!("CARGO_MANIFEST_DIR"));
    let ori_path = cur_path.clone() + "/data-original:/data-original";
    let vol_path = cur_path.clone() + "/volume:/data";
    let vol_comm = cur_path.clone() + "/source_files/execution.sh:/execution.sh";
    let gen_path = cur_path.clone() + "/generator:/generator";

    let mut container = Command::new("docker");
    container.args(["run", "-d",
        "--pid=host",
        "--privileged",
        "--network=host",
        "--device=/dev/isgx",
        "-v", ori_path.as_str(),
        "-v", vol_path.as_str(),
        "-v", vol_comm.as_str(),
        "-v", gen_path.as_str(),
        "--name", "executor",
        image_id.as_str()]);
    container
}

/// Main function to run the functions in the modul. Use ptrace to traces the system calls of the child process, which is created 
/// to run the container
pub fn run_executor() {
    // create docker image with specific setting
    let image_id = create_image();
    // create the container command with base on the created image file, to run the traced traget program
    let mut container = create_container(image_id);
    // execute the container command
    let target = container.output().expect("Failed at run container command");
    if !target.status.success() {
        panic!("Failed at generate the target container: {}", String::from_utf8(target.stderr).unwrap());
    }
    let mut container_id = String::from_utf8(target.stdout).unwrap();
    container_id.pop();
    // Check whether the container are finished and stopped
    let mut flag = String::from("true");
    while !flag.eq("'false'\n") {
        let is_running = Command::new("docker")
            .args(["inspect", "--format", "'{{.State.Running}}'", container_id.as_str()])
            .output()
            .unwrap();
        flag = String::from_utf8(is_running.clone().stdout).unwrap();
    }
    
    // Remove the tested container
    // let remove_container = Command::new("docker")
    //     .args(["rm", "executor"])
    //     .spawn()
    //     .unwrap();
    
}
