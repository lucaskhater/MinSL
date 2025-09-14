use core::ptr::null_mut;

use crate::cstr::strlen;

pub unsafe fn split(mut s: *mut u8, c: u8, buf: &mut [*mut u8]) -> usize {
    if *s == 0 {
        buf[0] = null_mut();
        return 0;
    }

    let mut i = 0;
    while *s != 0 && i < buf.len() - 1 {
        while *s == c {
            *s = 0;
            s = s.add(1);
        }

        buf[i] = s;
        i += 1;

        while *s != 0 && *s != c {
            s = s.add(1);
        }

        if *s != 0 {
            *s = 0;
            s = s.add(1);
        }
    }

    buf[i] = null_mut();
    i
}

pub unsafe fn trim(mut s: *mut u8) -> *mut u8 {
    while is_space(*s) {
        s = s.add(1);
    }

    if *s == 0 {
        return s;
    }

    let mut i = strlen(s) - 1;
    while is_space(*s.add(i)) {
        *s.add(i) = 0;
        i -= 1;
    }
    s
}

#[inline(always)]
unsafe fn is_space(c: u8) -> bool {
    matches!(c, b' ' | b'\n' | b'\r' | b'\t' | 0x0B | 0x0C)
}
