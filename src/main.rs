#![no_std]
#![no_main]

mod cstr;
mod env;
mod lexer;
mod memlibc;
mod path;
mod siglibc;
mod syscalls;

use crate::{
    cstr::*,
    env::getenv,
    lexer::{split, trim},
    path::path_lookup,
    syscalls::*,
};
use core::{mem::zeroed, ptr::null_mut};

const STR_MAX_LENGTH: usize = 8192;
const ARGVE_MAX_LENGTH: usize = 512;
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
        let mut usrin: [u8; STR_MAX_LENGTH] = [0; STR_MAX_LENGTH];
        let mut cmd: [u8; STR_MAX_LENGTH] = [0; STR_MAX_LENGTH];
        let mut path: [u8; STR_MAX_LENGTH] = [0; STR_MAX_LENGTH];

        unsafe {
            let argve: &mut [*mut u8] = &mut [null_mut(); ARGVE_MAX_LENGTH];

            _write(
                1,
                prompt.as_ptr() as *const u8,
                strlen(prompt.as_ptr() as *const u8),
            );
            _read(0, usrin.as_mut_ptr().cast(), STR_MAX_LENGTH);

            split(trim(usrin.as_mut_ptr()), b' ', argve);
            match strchr(usrin.as_ptr(), b'/') {
                None => {
                    strcpy(getenv(envp, b"PATH\0".as_ptr()).unwrap(), path.as_mut_ptr());
                    path_lookup(path.as_mut_ptr(), *argve.as_ptr(), cmd.as_mut_ptr());
                }
                Some(_) => {
                    strcpy(*argve.as_ptr(), cmd.as_mut_ptr());
                }
            }

            let pid = _fork();
            if pid == 0 {
                _execve(cmd.as_ptr(), argve.as_ptr() as *const *const u8, envp);
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
