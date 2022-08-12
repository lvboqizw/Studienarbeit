use libc::*;
use syscalls::*;
use rand::Rng;

const BUFF_SIZE: usize = 512;

pub fn read_write() {
    let path = "/home/zw/Documents/testforSa/testCode/src/data/test01\0";
    let buff = vec![b' '; BUFF_SIZE];
    let flag = rand_falgs();
    let file_dis = unsafe{ syscall2(Sysno::open, path.as_ptr() as usize, flag as usize)}.unwrap();
    let read_len = unsafe { syscall3(Sysno::read, file_dis, buff.as_ptr() as usize, BUFF_SIZE)}.unwrap();
    let write_len = unsafe{ syscall3(Sysno::write, 1, buff.as_ptr() as usize, read_len )};
    let result = unsafe{ syscall1(Sysno::close, file_dis)}.unwrap();
}

fn rand_falgs() -> c_int {
    let mut rng = rand::thread_rng();
    let read_mode = rng.gen_range(0..3);
    let flag;
    match read_mode {
        0 => flag = O_RDONLY,
        1 => flag = O_WRONLY,
        _ => flag = O_RDWR,
    }
    flag
}
