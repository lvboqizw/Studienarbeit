use syscalls::*;
use libc::*;
use std::ptr;
use rand::Rng;
use std::fs;

// const BUF_SIZE: usize = 200;

fn main() {
    // let path = "/data/read_file.txt\0";
    // // let w_path = "/data/write_file.txt\0";

    // let path_plain = "/operation/read_file.txt\0";
    // // let w_path_plain = "/operation/write_file.txt\0";
    // let mut rng = rand::thread_rng();
    // let mut read_len: usize;
    // let mut buff = vec![b' '; BUF_SIZE];                     // the bpftrace buffer is limited at 200 bytes
    // let mut buff_plain = vec![b' '; BUF_SIZE];
    // let buff_ptr = buff.as_mut_ptr();
    // let buff_plain_ptr = buff_plain.as_mut_ptr();
    // let flag = O_RDWR;
    // let file_dis = unsafe{ syscall2(Sysno::open, path.as_ptr() as usize, flag as usize)}.unwrap();
    // // let write_dis = unsafe{ syscall2(Sysno::open, w_path.as_ptr() as usize, flag as usize)}.unwrap();
    // let file_plain = unsafe{ syscall2(Sysno::open, path_plain.as_ptr() as usize, flag as usize)}.unwrap();
    // // let write_plain = unsafe{ syscall2(Sysno::open, w_path_plain.as_ptr() as usize, flag as usize)}.unwrap();

    // /*read, write */
    // read_len = rng.gen_range(0..200);
    // //  read the file protected by fspf, the system call should be encrypted
    // let mut read: usize = unsafe { syscall3(Sysno::read, file_dis, buff.as_ptr() as usize,  read_len)}.unwrap();
    // //  read the unprotecetd file which contains the same content as protected file.
    // let mut _read_plain: usize = unsafe { syscall3(Sysno::read, file_plain, buff_plain.as_ptr() as usize,  read_len)}.unwrap();
    // while read != 0 {
    //     // let _write = unsafe{ syscall3(Sysno::write, write_dis, buff.as_ptr() as usize, read)}.unwrap();
    //     // let _write_plain = unsafe{ syscall3(Sysno::write, write_plain, buff_plain.as_ptr() as usize, read_plain)}.unwrap();
    //     unsafe{ptr::write_bytes(buff_ptr, 0, BUF_SIZE);}
    //     unsafe{ptr::write_bytes(buff_plain_ptr, 0, BUF_SIZE);}
    //     read_len = rng.gen_range(0..200);
    //     read = unsafe { syscall3(Sysno::read, file_dis, buff.as_ptr() as usize,  read_len)}.unwrap();
    //     _read_plain = unsafe { syscall3(Sysno::read, file_plain, buff_plain.as_ptr() as usize,  read_len)}.unwrap();
    // }

    // let _result = unsafe{ syscall1(Sysno::close, file_dis)}.unwrap();
    // // let _result = unsafe{ syscall1(Sysno::close, write_dis)}.unwrap();
    
    open_folder();
}

fn open_folder() {
    let dir_org = "/data/test_files";
    let entries = fs::read_dir(dir_org).unwrap();
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            let mut path = path.to_str().unwrap().to_string();
            let mut buff = vec![b'0'; 200];
            let buff_ptr = buff.as_mut_ptr();
            let flag = O_RDWR;
            path += "\0";
            let file_dis = unsafe{ syscall2(Sysno::open, path.as_ptr() as usize, flag as usize)}.unwrap();
            let mut rng = rand::thread_rng();
            // let mut read_len: usize = rng.gen_range(0..200);
            let read_len = 200;

            let mut read = 1;
            let mut content: Vec<u8> = Vec::new();
            while read != 0 {
                read = unsafe{ syscall3(Sysno::read, file_dis, buff.as_ptr() as usize, read_len)}.unwrap();
                if read == 0 {
                    break;
                }
                let slice = &buff[0 .. read_len];
                content.extend_from_slice(slice);
                
                // read_len = rng.gen_range(0..200);
                unsafe{ptr::write_bytes(buff_ptr, 0, buff.len())};
            }
            let _result = unsafe{ syscall1(Sysno::close, file_dis)}.unwrap();
        }
    }
}
