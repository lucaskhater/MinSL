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

pub unsafe fn strchr(s: *const u8, c: u8) -> Option<*const u8> {
    let mut i = 0;
    loop {
        let x = *s.add(i);
        if x == c {
            return Some(s.add(i));
        }
        if x == 0 {
            return None;
        }
        i += 1;
    }
}

pub unsafe fn strjoin(s1: *const u8, s2: *const u8, buf: *mut u8) -> usize {
    let mut i = 0;
    while *s1.add(i) != 0 {
        *buf.add(i) = *s1.add(i);
        i += 1;
    }
    let mut j = 0;
    while *s2.add(j) != 0 {
        *buf.add(i + j) = *s2.add(j);
        j += 1;
    }
    *buf.add(i + j) = 0;
    i + j
}

pub unsafe fn strcpy(s: *const u8, target: *mut u8) -> i32 {
    if s.is_null() || *s == 0 {
        *target = 0;
        return -1;
    }

    let mut i = 0;
    while *s.add(i) != 0 {
        *target.add(i) = *s.add(i);
        i += 1;
    }
    *target.add(i) = 0;
    0
}
