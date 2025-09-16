use crate::siglibc::siginfo_t;

unsafe extern "C" {
    fn _write(fd: i32, buf: *const u8, count: usize) -> isize;
    fn _read(fd: i32, buf: *mut u8, count: usize) -> isize;
    fn _access(pathname: *const u8, mode: i32) -> i32;
    fn _execve(path: *const u8, argv: *const *const u8, envp: *const *const u8) -> i32;
    fn _fork() -> i32;
    fn _waitid(idtype: i32, id: i32, infop: *mut siginfo_t, options: i32) -> i32;
    fn _exit(status: i32) -> !;
}

pub fn write(fd: i32, buf: &[u8]) -> usize {
    let ret = unsafe { _write(fd, buf.as_ptr(), buf.len()) };
    ret as usize
}

pub fn read(fd: i32, buf: &mut [u8]) -> usize {
    let ret = unsafe { _read(fd, buf.as_mut_ptr(), buf.len()) };
    ret as usize
}

pub fn access(pathname: &[u8], mode: i32) -> i32 {
    let ret = unsafe { _access(pathname.as_ptr(), mode) };
    ret
}

pub fn execve(path: &[u8], argv: *const *const u8, envp: *const *const u8) -> i32 {
    let ret = unsafe { _execve(path.as_ptr(), argv, envp) };
    ret
}

pub fn fork() -> i32 {
    let ret = unsafe { _fork() };
    ret
}

pub fn waitid(idtype: i32, id: i32, infop: &mut siginfo_t, options: i32) -> i32 {
    let ret = unsafe { _waitid(idtype, id, infop, options) };
    ret
}

pub fn exit(status: i32) -> ! {
    unsafe { _exit(status) };
}
