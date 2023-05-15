use std::io::{Write, BufRead, BufReader, Read};
use std::net::TcpStream;
use std::fs::File;
use std::str;

fn main() {
  let mut stream = TcpStream::connect("127.0.0.1:9000").expect("connect failed");

  let file = File::open("/operation/message.txt").unwrap();
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
    println!("read {} bytes: {:?}", len, str::from_utf8(&buf[..len]));
  }
}