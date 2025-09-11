use crate::cstr::strlen;

const MAX_TOKS: usize = 64;

pub unsafe fn split(mut s: *mut u8, c: u8) -> *const *mut u8 {
    static mut BUF: [*mut u8; MAX_TOKS] = [core::ptr::null_mut(); MAX_TOKS];

    if *s == 0 {
        return BUF.as_ptr();
    }

    let mut i = 0;
    while *s != 0 && i < MAX_TOKS - 1 {
        while *s == c {
            *s = 0;
            s = s.add(1);
        }

        BUF[i] = s;
        i += 1;

        while *s != 0 && *s != c {
            s = s.add(1);
        }

        if *s != 0 {
            *s = 0;
            s = s.add(1);
        }
    }

    BUF[i] = core::ptr::null_mut();
    BUF.as_ptr()
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
