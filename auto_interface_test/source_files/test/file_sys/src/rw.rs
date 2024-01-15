use syscalls::*;
use libc::*;
use std::fs::{self, DirEntry};

const BUF_SIZE: usize = 200;

fn r_w(entry: DirEntry, mut write_p: String) {
    let mut path = entry.path().to_str().unwrap().to_string();
    let v: Vec<&str> = path.split("/").collect();
    write_p = write_p + "/" + v[v.len() - 1]+"\0";

    // Open file be read
    let r_flag = O_RDWR;
    path += "\0";
    let r_fd = unsafe{syscall2(Sysno::open, path.as_ptr() as usize, r_flag as usize)}.unwrap();

    // Open file to write
    let w_flag = O_RDWR | O_CREAT;
    let w_fd = unsafe{ syscall2(Sysno::open, write_p.as_ptr() as usize, w_flag as usize)}.unwrap();

    let mut buff = vec![b'0'; BUF_SIZE];
    let mut read_res: usize = 1;
    while read_res != 0 {
        read_res = unsafe{
            syscall3(Sysno::read, r_fd, buff.as_ptr() as usize, BUF_SIZE)
        }.unwrap();
        if read_res == 0 {
            break;
        }

        let _write = unsafe{
            syscall3(Sysno::write, w_fd, buff.as_ptr() as usize, read_res)
        }.unwrap();

        for i in 0 .. BUF_SIZE {
            buff[i] = b'0';
        }
    }
    let _result = unsafe{ syscall1(Sysno::close, r_fd)}.unwrap();
    let _result = unsafe{ syscall1(Sysno::close, w_fd)}.unwrap();
}

pub fn re_we() {
    let re_p = "/R_en";
    let we_p = "/W_en/Re_We".to_string();
    let entries = fs::read_dir(re_p).unwrap();
    for entry in entries {
        if let Ok(entry) = entry {
            r_w(entry, we_p.clone());
        }
    }
}

pub fn ru_we() {
    let re_p = "/R_ue";
    let we_p = "/W_en/Ru_We".to_string();
    let entries = fs::read_dir(re_p).unwrap();
    for entry in entries {
        if let Ok(entry) = entry {
            r_w(entry, we_p.clone());
        }
    }
}

pub fn re_wu() {
    let re_p = "/R_en";
    let we_p = "/W_ue/Re_Wu".to_string();
    let entries = fs::read_dir(re_p).unwrap();
    for entry in entries {
        if let Ok(entry) = entry {
            r_w(entry, we_p.clone());
        }
    }
}

pub fn ru_wu() {
    let re_p = "/R_ue";
    let we_p = "/W_ue/Re_Wu".to_string();
    let entries = fs::read_dir(re_p).unwrap();
    for entry in entries {
        if let Ok(entry) = entry {
            r_w(entry, we_p.clone());
        }
    }
}