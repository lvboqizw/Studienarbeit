use std::io::{Write, BufRead, BufReader, Read};
use std::net::{TcpListener, TcpStream};
use anyhow::Result;
use std::fs::File;


fn handle_client(mut stream: TcpStream) -> Result<()> {
    let mut buf = [0; 128];
    loop {
        let len = stream.read(&mut buf).unwrap();
        if len == 0 {
            break;
        }
    }
    let file = File::open("/R_ue/message.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let input = line.unwrap();
        let size = input.len();
        stream
            .write(&input.as_bytes()[..size])
            .expect("write failed");
    }
    Ok(())
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8087").unwrap();

    for stream in listener.incoming() {
        if let Ok(_) = handle_client(stream.unwrap()){
            break;
        }
    }
}