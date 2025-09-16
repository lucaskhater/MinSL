use crate::{
    cstr::strjoin,
    lexer::{split, tokenize, trim, Span},
    syscalls::access,
};

const MAX_CMD_LENGTH: usize = 512;
const MAX_PATH: usize = 512;
//const F_OK: i32 = 0;
const X_OK: i32 = 1;
//const W_OK: i32 = 2;
//const R_OK: i32 = 4;

pub unsafe fn path_lookup<'a>(
    path: &'a mut [u8],
    s: &[u8],
    target: &'a mut [u8],
) -> Option<&'a mut [u8]> {
    let mut buf: [&[u8]; MAX_PATH] = [&[]; MAX_PATH];
    let mut span: [Span; MAX_PATH] = [Span::new(0, 0); MAX_PATH];

    let Some(spath) = trim(path) else {
        return None;
    };
    split(spath, b':', &mut span);
    let pathc = tokenize(spath, &span, &mut buf);

    let mut i = 0;
    while i < pathc {
        let cmd: &mut [u8] = &mut [0; MAX_CMD_LENGTH];
        strjoin(buf[i], b"/\0", cmd);
        strjoin(cmd, s, target);

        if file_access(target, X_OK) {
            return Some(target);
        }

        i += 1;
    }
    None
}

#[inline(always)]
fn file_access(path: &[u8], mode: i32) -> bool {
    match access(path, mode) {
        0 => true,
        _ => false,
    }
}
