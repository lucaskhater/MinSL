use crate::siglibc::siginfo_t;

unsafe extern "C" {
    pub unsafe fn _write(fd: i32, buf: *const u8, count: usize) -> isize;
    pub unsafe fn _read(fd: i32, buf: *mut u8, count: usize) -> isize;
    pub unsafe fn _access(pathname: *const u8, mode: i32) -> i32;
    pub unsafe fn _execve(path: *const u8, argv: *const *const u8, envp: *const *const u8) -> i32;
    pub unsafe fn _fork() -> i32;
    pub unsafe fn _waitid(idtype: i32, id: i32, infop: *mut siginfo_t, options: i32) -> i32;
    pub unsafe fn _exit(status: i32) -> !;
}
