use syscalls::*;
use libc::*;
use std::ptr;
use rand::Rng;

const BUF_SIZE: usize = 200;

fn main() {
    let path = "/data/read_file.txt\0";
    // let w_path = "/data/write_file.txt\0";

    let path_plain = "/operation/read_file.txt\0";
    // let w_path_plain = "/operation/write_file.txt\0";
    let mut rng = rand::thread_rng();
    let mut read_len: usize;
    let mut buff = vec![b' '; BUF_SIZE];                     // the bpftrace buffer is limited at 200 bytes
    let mut buff_plain = vec![b' '; BUF_SIZE];
    let buff_ptr = buff.as_mut_ptr();
    let buff_plain_ptr = buff_plain.as_mut_ptr();
    let flag = O_RDWR;
    let file_dis = unsafe{ syscall2(Sysno::open, path.as_ptr() as usize, flag as usize)}.unwrap();
    // let write_dis = unsafe{ syscall2(Sysno::open, w_path.as_ptr() as usize, flag as usize)}.unwrap();
    let file_plain = unsafe{ syscall2(Sysno::open, path_plain.as_ptr() as usize, flag as usize)}.unwrap();
    // let write_plain = unsafe{ syscall2(Sysno::open, w_path_plain.as_ptr() as usize, flag as usize)}.unwrap();

    /*read, write */
    read_len = rng.gen_range(0..200);
    //  read the file protected by fspf, the system call should be encrypted
    let mut read: usize = unsafe { syscall3(Sysno::read, file_dis, buff.as_ptr() as usize,  read_len)}.unwrap();
    //  read the unprotecetd file which contains the same content as protected file.
    let mut _read_plain: usize = unsafe { syscall3(Sysno::read, file_plain, buff_plain.as_ptr() as usize,  read_len)}.unwrap();
    while read != 0 {
        // let _write = unsafe{ syscall3(Sysno::write, write_dis, buff.as_ptr() as usize, read)}.unwrap();
        // let _write_plain = unsafe{ syscall3(Sysno::write, write_plain, buff_plain.as_ptr() as usize, read_plain)}.unwrap();
        unsafe{ptr::write_bytes(buff_ptr, 0, BUF_SIZE);}
        unsafe{ptr::write_bytes(buff_plain_ptr, 0, BUF_SIZE);}
        read_len = rng.gen_range(0..200);
        read = unsafe { syscall3(Sysno::read, file_dis, buff.as_ptr() as usize,  read_len)}.unwrap();
        _read_plain = unsafe { syscall3(Sysno::read, file_plain, buff_plain.as_ptr() as usize,  read_len)}.unwrap();
    }

    let _result = unsafe{ syscall1(Sysno::close, file_dis)}.unwrap();
    // let _result = unsafe{ syscall1(Sysno::close, write_dis)}.unwrap();
}
