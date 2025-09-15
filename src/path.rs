use core::ptr::null_mut;

use crate::{
    cstr::strjoin,
    lexer::{split, trim},
    syscalls::access,
};

const MAX_CMD_LENGTH: usize = 512;
const MAX_PATH: usize = 512;
//const F_OK: i32 = 0;
const X_OK: i32 = 1;
//const W_OK: i32 = 2;
//const R_OK: i32 = 4;

pub unsafe fn path_lookup(path: *mut u8, s: *const u8, target: *mut u8) -> Option<*mut u8> {
    let buf: &mut [*mut u8] = &mut [null_mut(); MAX_PATH];
    let pathc = split(trim(path), b':', buf);

    let mut i = 0;
    while i < pathc {
        let cmd: &mut [u8] = &mut [0; MAX_CMD_LENGTH];
        strjoin(buf[i], b"/\0".as_ptr(), cmd.as_mut_ptr());
        strjoin(cmd.as_mut_ptr(), s, target);

        if file_access(target.as_ref().unwrap(), X_OK) {
            return Some(target);
        }

        i += 1;
    }
    None
}

#[inline(always)]
fn file_access(path: &u8, mode: i32) -> bool {
    match access(path, mode) {
        0 => true,
        _ => false,
    }
}
