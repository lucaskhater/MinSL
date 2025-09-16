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
    lexer::{split, tokenize, trim, Span},
    path::path_lookup,
    siglibc::siginfo_t,
    syscalls::*,
};

const STR_MAX_LENGTH: usize = 8192;
pub const ARGVE_MAX_LENGTH: usize = 512;
const P_ALL: i32 = 0;
const WEXITED: i32 = 4;

#[no_mangle]
pub fn _main(rsp: *const usize) -> ! {
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

        let mut argvs: [Span; ARGVE_MAX_LENGTH] = [Span::new(0, 0); ARGVE_MAX_LENGTH];
        let mut argve: [&[u8]; ARGVE_MAX_LENGTH] = [&[]; ARGVE_MAX_LENGTH];

        write(1, prompt);
        read(0, &mut usrin);

        let Some(s) = trim(&mut usrin[..]) else {
            continue;
        };
        split(s, b' ', &mut argvs);
        tokenize(s, &argvs, &mut argve);

        unsafe {
            match strchr(&usrin, b'/') {
                None => {
                    strcpy(
                        getenv(envp.as_ref().unwrap(), b"PATH\0").unwrap(),
                        &mut path,
                    );
                    path_lookup(&mut path, &argve[0], &mut cmd);
                }
                Some(_) => {
                    strcpy(*argve.as_ptr(), &mut cmd);
                }
            }
        }

        let pid = fork();
        if pid == 0 {
            execve(&cmd, argve.as_ptr() as *const *const u8, envp);
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
