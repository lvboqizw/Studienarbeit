use syscalls::*;
use libc::*;
use std::ptr;
use rand::Rng;
use std::fs;

// const BUF_SIZE: usize = 200;

fn main() {
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
