use libc::{O_CREAT, O_RDWR, O_NONBLOCK};
use syscalls::{raw_syscall, Sysno, syscall};

mod syscall_open;
mod syscall_read;

use self::syscall_open::SysOpen;
use self::syscall_read::SysRead;

// mod sys_strcut;


pub fn function() {
    // let path = "test01.txt";
    // let sys_open = SysOpen::new(
    //     2,
    //     0 as usize,
    //     path.as_ptr() as usize, 
    //     (O_RDWR) as usize,
    //     00700 as usize,
    // );
    // sys_open.go();
    let fd = unsafe {raw_syscall!(Sysno::open, "test01.txt".as_ptr() as usize, O_RDWR | O_NONBLOCK)};
    println!("fd: {:?}", fd);
    let sys_read = SysRead::new(
        0,
        fd,
    );
    sys_read.go();
    
    // let buff = vec![b'a'; 10];
    // let res = unsafe {syscall!(Sysno::write, 1, buff.as_ptr() as usize , 10)};
    // println!("{:?}", res);
    // println!("buff: {:?}", buff);
}