const MAX_TOKS: usize = 64;

pub unsafe fn split(mut s: *mut u8, c: u8) -> *const *mut u8 {
    static mut BUF: [*mut u8; MAX_TOKS] = [core::ptr::null_mut(); MAX_TOKS];

    if *s == 0 {
        return BUF.as_ptr();
    }

    let mut count = 0;

    while *s != 0 && count < MAX_TOKS - 1 {
        while *s == c {
            *s = 0;
            s = s.add(1);
        }

        if *s == 0 {
            break;
        }

        BUF[count] = s;
        count += 1;

        while *s != 0 && *s != c {
            s = s.add(1);
        }

        if *s != 0 {
            *s = 0;
            s = s.add(1);
        }
    }

    BUF[count] = core::ptr::null_mut();
    BUF.as_ptr()
}

