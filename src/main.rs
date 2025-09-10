#![no_std]
#![no_main]

mod cstr;
mod lexer;
mod memlibc;
mod siglibc;
mod syscalls;

use crate::{cstr::*, syscalls::*};
use core::mem::zeroed;

const CMD_MAX_LENGTH: usize = 255;
const P_ALL: i32 = 0;
const WEXITED: i32 = 4;

#[no_mangle]
pub extern "C" fn _main() -> ! {
    let prompt: &[u8] = b"MinSL:$ \0";
    let mut cmd: [u8; CMD_MAX_LENGTH] = [0; CMD_MAX_LENGTH];
    let mut count: isize;
 
    loop {
        unsafe {
            _write(
                1,
                prompt.as_ptr() as *const u8,
                strlen(prompt.as_ptr() as *const u8),
            );
            count = _read(0, cmd.as_mut_ptr().cast(), CMD_MAX_LENGTH);
        }

        cmd[count as usize - 1] = 0;

        unsafe {
            let pid = _fork();

            if pid == 0 {
                let argv: [*const u8; 2] = [cmd.as_ptr() as *const u8, core::ptr::null()];

                let envp: [*const u8; 1] = [core::ptr::null()];

                _execve(
                    cmd.as_ptr() as *const u8,
                    argv.as_ptr() as *const *const u8,
                    envp.as_ptr() as *const *const u8,
                );
                break;
            } else {
                let mut siginfo = zeroed();
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
