#![no_std]
#![no_main]

mod cstr;
mod lexer;
mod env;
mod memlibc;
mod siglibc;
mod syscalls;

use crate::{cstr::*, env::getenv, lexer::{split, trim}, syscalls::*};
use core::{mem::zeroed};

const CMD_MAX_LENGTH: usize = 255;
const P_ALL: i32 = 0;
const WEXITED: i32 = 4;

#[no_mangle]
pub extern "C" fn _main(rsp: *const usize) -> ! {
    let argc: usize;
    let argv: *const *const u8;
    let envp: *const *const u8;

    unsafe {
        argc = *rsp;
        argv = rsp.add(1) as *const *const u8;
        envp = argv.add(argc + 1) as *const *const u8;
    }

    let prompt: &[u8] = b"MinSL:$ \0";

    loop {
        let mut cmd: [u8; CMD_MAX_LENGTH] = [0; CMD_MAX_LENGTH];
        let argve: *const *mut u8;

        unsafe {
            _write(
                1,
                prompt.as_ptr() as *const u8,
                strlen(prompt.as_ptr() as *const u8),
            );
            _read(0, cmd.as_mut_ptr().cast(), CMD_MAX_LENGTH);

            argve = split(trim(cmd.as_mut_ptr()), b' ');

            let pid = _fork();
            if pid == 0 {
                _execve(
                    *argve as *const u8,
                    argve as *const *const u8,
                    envp,
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
