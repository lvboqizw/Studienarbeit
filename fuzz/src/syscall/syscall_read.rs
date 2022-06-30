use syscalls::{Sysno, syscall_args, syscall};

pub struct SysRead {
    nr: u32,
    fd: usize,
    buf: Vec<u8>,
    count: usize,
}

impl SysRead{
    /// a0: number, a1: fd
    pub fn new(a0: u32, a1: usize) -> Self{
        SysRead {
            nr: a0,
            fd: a1,
            buf: vec![b' '; 10],
            count: 10 as usize,
        }
    } 

    pub fn go(&self) {
        let arg = syscall_args!(self.fd, self.buf.as_ptr() as usize, self.count);
        let res = unsafe {syscall!(Sysno::read, arg.arg0, arg.arg1, arg.arg2)};
        println!("res: {:?}", res);
        println!("after read: {:?}", self.buf);
    }
}
