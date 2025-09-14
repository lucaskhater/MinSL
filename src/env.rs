use crate::cstr::*;

pub unsafe fn getenv(envp: *const *const u8, s: *const u8) -> Option<*const u8> {
    if *s == 0 || (*envp).is_null() || envp.is_null() {
        return None;
    }

    let mut i = 0;
    while !(*envp.add(i)).is_null() {
        let var = *envp.add(i);
        if strncmp(var, s, strlen(s)) == 0 && *var.add(strlen(s)) == b'=' {
            return Some(var.add(strlen(s) + 1));
        }
        i += 1;
    }

    None
}
