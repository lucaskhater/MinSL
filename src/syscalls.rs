use core::ffi::{c_char, c_int, c_void};
use crate::libc::siginfo_t;

unsafe extern "C" {
    pub unsafe fn _write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    pub unsafe fn _read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    pub unsafe fn _execve(
        path: *const c_char,
        argv: *const *const c_char,
        envp: *const *const c_char,
    ) -> c_int;
    pub unsafe fn _fork() -> i32;
    pub unsafe fn _waitid(idtype: i32, id: i32, infop: *mut siginfo_t, options: c_int) -> i32;
    pub unsafe fn _exit(status: c_int) -> !;
}