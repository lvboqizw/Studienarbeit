use std::io::{Write, BufRead, BufReader, Read};
use std::net::TcpStream;
use std::fs::File;

fn main() {
  let mut stream = TcpStream::connect("127.0.0.1:8087").expect("connect failed");

  let file = File::open("/R_ue/message.txt").unwrap();
  let reader = BufReader::new(file);

  for line in reader.lines() {
    let input = line.unwrap();
    let size = input.len();
    stream
      .write(&input.as_bytes()[..size])
      .expect("write failed");
  }
  stream.shutdown(std::net::Shutdown::Write).unwrap();

  let mut buf = [0; 128];
  loop {
    let len = stream.read(&mut buf).unwrap();
    if len == 0 {
        break;
    }
  }
}