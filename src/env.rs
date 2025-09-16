use crate::cstr::*;

pub unsafe fn getenv<'a>(envp: &[&'a [u8]], s: &[u8]) -> Option<&'a [u8]> {
    if s[0] == 0 || envp[0].is_empty() {
        return None;
    }

    let mut i = 0;
    while !envp[i].is_empty() {
        let var = envp[i];
        if strncmp(var, s, s.len()) == 0 && var[s.len()] == b'=' {
            return Some(&var[(s.len() + 1)..]);
        }
        i += 1;
    }

    None
}
