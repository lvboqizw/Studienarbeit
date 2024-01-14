use syscalls::*;
use libc::*;
use std::{ptr, fs};
use rand::Rng;

const BUF_SIZE: usize = 200;

pub fn read_write() {
    let dir_org = "/data/test_files";
    let path_write = "/data/write_file\0";
    let entries = fs::read_dir(dir_org).unwrap();
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            let mut path = path.to_str().unwrap().to_string();
            let mut buff = vec![b'0'; BUF_SIZE];
            let buff_ptr = buff.as_mut_ptr();
            let flag = O_RDWR;
            let w_flag = O_RDWR | O_CREAT;
            path += "\0";
            let file_dis = unsafe{ syscall2(Sysno::open, path.as_ptr() as usize, flag as usize)}.unwrap();
            let file_write = unsafe{ syscall2(Sysno::open, path_write.as_ptr() as usize, w_flag as usize)}.unwrap();
            let mut rng = rand::thread_rng();
            let mut read_len: usize = rng.gen_range(0..BUF_SIZE);

            let mut read = 1;
            while read != 0 {
                read = unsafe{ syscall3(Sysno::read, file_dis, buff.as_ptr() as usize, read_len)}.unwrap();
                if read == 0 {
                    break;
                }
                let _write = unsafe{ syscall3(Sysno::write, file_write, buff.as_ptr() as usize, read)}.unwrap();
                
                read_len = rng.gen_range(0..200);
                unsafe{ptr::write_bytes(buff_ptr, 0, buff.len())};
            }
            let _result = unsafe{ syscall1(Sysno::close, file_dis)}.unwrap();
            let _result = unsafe{ syscall1(Sysno::close, file_write)}.unwrap();
        }
    }
}

pub fn read_write_v() {
    let dir_org = "/data/test_files";
    let path_write = "/data/write_file\0";
    let entries = fs::read_dir(dir_org).unwrap();
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            let mut path = path.to_str().unwrap().to_string();
            let mut buff = vec![b'0'; BUF_SIZE];
            let buff_ptr = buff.as_mut_ptr();
            let iovs1 = Iovec {
                _iov_base: buff.as_ptr() as usize,
                _iov_len: BUF_SIZE,
            };
            let iovs = vec![iovs1.clone()];


            let flag = O_RDWR;
            let w_flag = O_RDWR | O_CREAT;
            path += "\0";
            let file_dis = unsafe{ syscall2(Sysno::open, path.as_ptr() as usize, flag as usize)}.unwrap();
            let file_write = unsafe{ syscall2(Sysno::open, path_write.as_ptr() as usize, w_flag as usize)}.unwrap();
            // let mut rng = rand::thread_rng();
            // let mut read_len: usize = rng.gen_range(0..BUF_SIZE);

            let mut read = 1;
            while read != 0 {
                read = unsafe{ syscall3(Sysno::readv, file_dis, iovs.as_ptr() as usize, 1)}.unwrap();
                if read == 0 {
                    break;
                }
                let _write = unsafe{ syscall3(Sysno::writev, file_write, iovs.as_ptr() as usize, 1)}.unwrap();
                
                // read_len = rng.gen_range(0..200);
                unsafe{ptr::write_bytes(buff_ptr, 0, buff.len())};
            }
            let _result = unsafe{ syscall1(Sysno::close, file_dis)}.unwrap();
            let _result = unsafe{ syscall1(Sysno::close, file_write)}.unwrap();
        }
    }
}

#[derive(Debug, Clone)]
struct Iovec {
    _iov_base: usize,
    _iov_len: usize,
}