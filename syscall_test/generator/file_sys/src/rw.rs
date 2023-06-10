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


// pub fn file_rwv() {
//     let mut buff = vec![b' '; BUF_SIZE];
//     let buff_ptr = buff.as_mut_ptr();
//     let iovs1 = Iovec {
//         _iov_base: buff.as_ptr() as usize,
//         _iov_len: BUF_SIZE,
//     };
//     let iovs = vec![iovs1.clone()];

//     let path = "/data/read_file.txt\0";
//     let w_path = "/data/write_file.txt\0";
//     let flag = O_RDWR;
//     let w_flag = O_RDWR | O_CREAT; 
//     let file_dis = unsafe{ syscall2(Sysno::open, path.as_ptr() as usize, flag as usize)}.unwrap();
//     let write_dis = unsafe{ syscall2(Sysno::open, w_path.as_ptr() as usize, w_flag as usize)}.unwrap();

//     /*  readv, writev */
//     let mut readv = unsafe{ syscall3(Sysno::readv, file_dis, iovs.as_ptr() as usize, 1)}.unwrap();
//     while readv != 0 {
//         let _writev = unsafe{ syscall3(Sysno::writev, write_dis, iovs.as_ptr() as usize, 1)}.unwrap();
//         unsafe{ptr::write_bytes(buff_ptr, 0, BUF_SIZE);}
//         readv = unsafe{ syscall3(Sysno::readv, file_dis, iovs.as_ptr() as usize, 1)}.unwrap();
//     }


//     /*  preadv, pwritev */
//     let mut offset = 0;
//     let mut preadv = unsafe{ syscall4(Sysno::preadv, file_dis, iovs.as_ptr() as usize, 1, offset)}.unwrap();
//     while preadv == 200 {
//         let _pwritev = unsafe{ syscall4(Sysno::pwritev, write_dis, iovs.as_ptr() as usize, 1, offset)}.unwrap();
//         unsafe{ptr::write_bytes(buff_ptr, 0, BUF_SIZE);}
//         offset += preadv;
//         preadv = unsafe{ syscall4(Sysno::preadv, file_dis, iovs.as_ptr() as usize, 1, offset)}.unwrap();
//     }

//     let _result = unsafe{ syscall1(Sysno::close, file_dis)}.unwrap();
//     let _result = unsafe{ syscall1(Sysno::close, write_dis)}.unwrap();
// }



// pub fn file_rw() {
//     let path = "/data/read_file.txt\0";
//     let w_path = "/data/write_file.txt\0";

//     let mut rng = rand::thread_rng();
//     let mut read_len: usize;
//     let mut buff = vec![b' '; BUF_SIZE];                     // the bpftrace buffer is limited at 200 bytes
//     let buff_ptr = buff.as_mut_ptr();
//     let flag = O_RDWR;
//     let file_dis = unsafe{ syscall2(Sysno::open, path.as_ptr() as usize, flag as usize)}.unwrap();
//     let write_dis = unsafe{ syscall2(Sysno::open, w_path.as_ptr() as usize, flag as usize)}.unwrap();

//     /*read, write */
//     read_len = rng.gen_range(0..200);
//     //  read the file protected by fspf, the system call should be encrypted
//     let mut read: usize = unsafe { syscall3(Sysno::read, file_dis, buff.as_ptr() as usize,  read_len)}.unwrap();
//     while read != 0 {
//         let _write = unsafe{ syscall3(Sysno::write, write_dis, buff.as_ptr() as usize, read)}.unwrap();
//         unsafe{ptr::write_bytes(buff_ptr, 0, BUF_SIZE);}
//         read_len = rng.gen_range(0..200);
//         read = unsafe { syscall3(Sysno::read, file_dis, buff.as_ptr() as usize,  read_len)}.unwrap();
//     }

//     /*  pread64, pwrite64 */
//     let mut offset = 0;
//     read_len = rng.gen_range(0..200);
//     let mut pread64: usize = unsafe { syscall4(Sysno::pread64, file_dis, buff.as_ptr() as usize, read_len, offset)}.unwrap();
//     while pread64 != 0 {
//         let _pwrite64 = unsafe { syscall4(Sysno::pwrite64, write_dis, buff.as_ptr() as usize, pread64, offset)}.unwrap();
//         unsafe{ptr::write_bytes(buff_ptr, 0, BUF_SIZE);}
//         offset += pread64;
//         read_len = rng.gen_range(0..200);
//         pread64 = unsafe { syscall4(Sysno::pread64, file_dis, buff.as_ptr() as usize, read_len, offset)}.unwrap();
//     }

//     let _result = unsafe{ syscall1(Sysno::close, file_dis)}.unwrap();
//     let _result = unsafe{ syscall1(Sysno::close, write_dis)}.unwrap();
// }

