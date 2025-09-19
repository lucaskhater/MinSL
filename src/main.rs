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
    env::{capture, getenv, Env},
    lexer::{split, tokenize, trim, Span},
    path::path_lookup,
    siglibc::SigInfo,
    syscalls::*,
};

const STR_MAX_LENGTH: usize = 8192;
pub const ARGVE_MAX_LENGTH: usize = 512;
const P_ALL: i32 = 0;
const WEXITED: i32 = 4;

#[no_mangle]
pub extern "C" fn _main(rsp: *const usize) -> ! {
    let mut bargv: [&[u8]; ARGVE_MAX_LENGTH + 1] = [&[]; ARGVE_MAX_LENGTH + 1];
    let mut benvp: [&[u8]; ARGVE_MAX_LENGTH + 1] = [&[]; ARGVE_MAX_LENGTH + 1];

    let mut env = Env {
        argc: 0,
        argv: &mut bargv[..],
        envp: &mut benvp[..],
    };
    let (_, _, envp) = capture(rsp, &mut env);

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
            match strchr(argve[0], b'/') {
                None => {
                    strcpy(getenv(envp, b"PATH").unwrap(), &mut path);
                    path_lookup(&mut path, &argve[0], &mut cmd);
                }
                Some(_) => {
                    strcpy(*argve.as_ptr(), &mut cmd);
                }
            }
        }

        let pid = fork();
        if pid == 0 {
            execve(
                &cmd,
                argve.as_ptr() as *const *const u8,
                envp.as_ptr() as *const *const u8,
            );
            break;
        } else {
            let mut siginfo = SigInfo {
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
