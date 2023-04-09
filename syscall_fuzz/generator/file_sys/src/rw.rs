use std::{fs, path::Path, io, process::Command, thread};
use syscalls::*;
use libc::*;

pub fn file_rw() {
    let path = "/data/read_file.txt\0";
    let w_path = "/data/write_file.txt\0";
    let buff = vec![b' '; 512];                     // the bpftrace buffer is limited at 512 bytes
    let flag = O_RDWR;
    let file_dis = unsafe{ syscall2(Sysno::open, path.as_ptr() as usize, flag as usize)}.unwrap();
    let write_dis = unsafe{ syscall2(Sysno::open, w_path.as_ptr() as usize, flag as usize)}.unwrap();

    let read = unsafe { syscall3(Sysno::read, file_dis, buff.as_ptr() as usize,  buff.len())}.unwrap();
    let write = unsafe{ syscall3(Sysno::write, write_dis, buff.as_ptr() as usize, read )}.unwrap();

    let pread64 = unsafe { syscall4(Sysno::pread64, file_dis, buff.as_ptr() as usize, buff.len(), 0)}.unwrap();
    let pwrite64 = unsafe { syscall4(Sysno::pwrite64, write_dis, buff.as_ptr() as usize, pread64, 0)}.unwrap();

    let result = unsafe{ syscall1(Sysno::close, file_dis)}.unwrap();
    let result = unsafe{ syscall1(Sysno::close, write_dis)}.unwrap();
}

pub fn file_rwv() {
    let buff1 = vec![b' '; 100];
    let buff2 = vec![b' '; 100];
    let buff3 = vec![b' '; 100];
    let buff4 = vec![b' '; 100];
    let buff5 = vec![b' '; 100];
    let iovs1 = Iovec {
        iov_base: buff1.as_ptr() as usize,
        iov_len: 100,
    };
    let iovs2 = Iovec {
        iov_base: buff2.as_ptr() as usize,
        iov_len: 100,
    };
    let iovs3 = Iovec {
        iov_base: buff3.as_ptr() as usize,
        iov_len: 100,
    };
    let iovs4 = Iovec {
        iov_base: buff4.as_ptr() as usize,
        iov_len: 100,
    };
    let iovs5 = Iovec {
        iov_base: buff5.as_ptr() as usize,
        iov_len: 100,
    };
    let mut iovs = vec![iovs1.clone(), iovs2.clone(), iovs3.clone(), iovs4.clone(), iovs5.clone()];

    let path = "/data/read_file.txt\0";
    let w_path = "/data/write_file.txt\0";
    let flag = O_RDWR;
    let w_flag = O_RDWR | O_CREAT; 
    let file_dis = unsafe{ syscall2(Sysno::open, path.as_ptr() as usize, flag as usize)}.unwrap();
    let write_dis = unsafe{ syscall2(Sysno::open, w_path.as_ptr() as usize, w_flag as usize)}.unwrap();

    let readv = unsafe{ syscall3(Sysno::readv, file_dis, iovs.as_ptr() as usize, 5)}.unwrap();
    let w_len = readv/buff1.len();
    let writev = unsafe{ syscall3(Sysno::writev, write_dis, iovs.as_ptr() as usize, w_len)}.unwrap();

    let preadv = unsafe{ syscall4(Sysno::preadv, file_dis, iovs.as_ptr() as usize, 5, 0)}.unwrap();
    let pw_len = preadv/buff1.len();
    let pwritev = unsafe{ syscall4(Sysno::pwritev, write_dis, iovs.as_ptr() as usize, pw_len, 0)}.unwrap();

    let result = unsafe{ syscall1(Sysno::close, file_dis)}.unwrap();
    let result = unsafe{ syscall1(Sysno::close, write_dis)}.unwrap();
}

#[derive(Debug, Clone)]
struct Iovec {
    iov_base: usize,
    iov_len: usize,
}