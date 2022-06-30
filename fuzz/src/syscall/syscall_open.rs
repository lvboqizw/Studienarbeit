use libc::{self, mode_t, O_RDWR};
use syscalls::{Sysno, syscall_args, syscall};

pub struct SysOpen {
    nr: u32,
    dirfd: usize,
    pathname: usize,
    flags: usize,
    mode: usize,
}

impl SysOpen {
    /// a0: nr, a1: dirfd, a3: pathname, a4: flags, a5: mode
    pub fn new(
        a0: u32, 
        a1: usize, 
        a2: usize,
        a3: usize,
        a4: usize) -> Self {
            SysOpen {
                nr: a0,
                dirfd: a1,
                pathname: a2,
                flags: a3,
                mode: a4
            }
    }

    pub fn go(&self) {
        let arg = syscall_args!(self.dirfd, self.pathname, self.flags, self.mode);
        let res = unsafe {syscall!(Sysno::open, arg.arg1, arg.arg2, arg.arg3)};
        println!("result: {:?}", res);
    }
}
