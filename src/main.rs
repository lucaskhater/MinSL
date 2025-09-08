#![no_std]
#![no_main]

mod syscalls;
mod libc;

use core::ffi::{c_char, c_int, c_void};
use crate::syscalls::*;

const CMD_MAX_LENGTH: usize = 255;
const P_ALL: i32 = 0;
const WEXITED: c_int = 4;

#[no_mangle]
pub extern "C" fn _main() -> ! {
    let prompt: &[u8] = b"$ ";
    let mut cmd: [u8; CMD_MAX_LENGTH] = [0; CMD_MAX_LENGTH];
    let mut count: isize;

    loop {
        unsafe {
            _write(1, prompt.as_ptr() as *const c_void, prompt.len());
            count = _read(0, cmd.as_mut_ptr().cast(), CMD_MAX_LENGTH);
        }

        cmd[count as usize - 1] = 0;

        unsafe {
            let pid = _fork();

            if pid == 0 {
                let argv: [*const c_char; 2] = [cmd.as_ptr() as *const c_char, core::ptr::null()];

                let envp: [*const c_char; 1] = [core::ptr::null()];

                _execve(
                    cmd.as_ptr() as *const c_char,
                    argv.as_ptr() as *const *const c_char,
                    envp.as_ptr() as *const *const c_char,
                );
                break;
            } else {
                let mut siginfo = core::mem::zeroed();
                _waitid(P_ALL, 0, &mut siginfo, WEXITED);
            }
        }
    }

    unsafe {
        _exit(0);
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    unsafe {
        _exit(-1);
    }
}