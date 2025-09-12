use crate::cstr::*;

pub unsafe fn getenv(mut envp: *const *const u8, s: &str) -> Option<*const u8> {
    loop {
        if s.is_empty() || (*envp).is_null() || **envp == 0 {
            return None;
        }

        if strncmp(*envp, s.as_ptr(), s.len()) == 0 && **envp.add(s.len()) == b'=' {
            return Some(*envp.add(s.len() + 1));
        }

        envp = envp.add(1);
    }
}
