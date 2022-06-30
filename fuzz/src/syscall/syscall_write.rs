use syscalls::{Sysno, syscall_args, syscall};

pub struct SysWrite<'a> {
    nr: u32,
    fd: usize,
    buf: &'a Vec<u8>,
    count: usize,
}

impl SysWrite<'_> {
    pub fn new(fd: usize, buf: &Vec<u8>, count: usize) -> SysWrite {
        SysWrite {
            nr: 1,
            fd,
            buf,
            count,
        }
    }

    pub fn go(&self) {
        let arg = syscall_args!(self.fd, self.buf.as_ptr() as usize, self.count);
        let res = unsafe{syscall!(Sysno::write, arg.arg0, arg.arg1, arg.arg2)};
        println!{"{:?}", res};
    }
}