use core::ptr::null;

pub unsafe fn strlen(mut s: *const u8) -> usize {
    let mut i: usize = 0;
    while *s != 0 {
        i += 1;
        s = s.add(1);
    }
    i
}

pub unsafe fn strcmp(s1: *const u8, s2: *const u8) -> i32 {
    let mut i = 0;
    loop {
        let x = *s1.add(i);
        let y = *s2.add(i);
        if x != y || x == 0 || y == 0 {
            return x as i32 - y as i32;
        }
        i += 1;
    }
}

pub unsafe fn strncmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    for i in 0..n {
        let x = *s1.add(i);
        let y = *s2.add(i);
        if x != y || x == 0 || y == 0 {
            return x as i32 - y as i32;
        }
    }
    0
}

pub unsafe fn strchr(s: *const u8, c: u8) -> *const u8 {
    let mut i = 0;
    loop {
        let x = *s.add(i);
        if x == c {
            return s.add(i);
        }
        if x == 0 {
            return null();
        }
        i += 1;
    }
}