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
    siglibc::siginfo_t,
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

        let argve: &mut [*mut u8] = &mut [null_mut(); ARGVE_MAX_LENGTH];

        write(1, prompt);
        read(0, &mut usrin);

        unsafe {
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
        }

        let pid = fork();
        if pid == 0 {
            execve(&cmd[0], argve.as_ptr() as *const *const u8, envp);
            break;
        } else {
            let mut siginfo = siginfo_t {
                si_signo: 0,
                si_errno: 0,
                si_code: 0,
                __pad0: 0,
                _sifields: [0; 112],
            };
            waitid(P_ALL, 0, &mut siginfo, WEXITED);
        }
    }

    exit(0);
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    exit(-1);
}
