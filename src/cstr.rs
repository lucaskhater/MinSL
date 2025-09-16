pub fn strlen(s: &[u8]) -> usize {
    return s.len();
}

pub fn strcmp(s1: &[u8], s2: &[u8]) -> i32 {
    let mut i = 0;
    loop {
        let x = s1[i];
        let y = s2[i];
        if x != y || x == 0 || y == 0 {
            return x as i32 - y as i32;
        }
        i += 1;
    }
}

pub fn strncmp(s1: &[u8], s2: &[u8], n: usize) -> i32 {
    for i in 0..n {
        let x = s1[i];
        let y = s2[i];
        if x != y || x == 0 || y == 0 {
            return x as i32 - y as i32;
        }
    }
    0
}

pub fn strchr(s: &[u8], c: u8) -> Option<usize> {
    let mut i = 0;
    loop {
        let x = s[i];
        if x == c {
            return Some(i);
        }
        if x == 0 {
            return None;
        }
        i += 1;
    }
}

pub fn strjoin(s1: &[u8], s2: &[u8], buf: &mut [u8]) -> usize {
    let mut i = 0;
    while s1[i] != 0 {
        buf[i] = s1[i];
        i += 1;
    }
    let mut j = 0;
    while s2[j] != 0 {
        buf[i + j] = s2[j];
        j += 1;
    }
    buf[i + j] = 0;
    i + j
}

pub fn strcpy(s: &[u8], target: &mut [u8]) -> i32 {
    if s.is_empty() || s[0] == 0 {
        target[0] = 0;
        return -1;
    }

    let mut i = 0;
    while s[i] != 0 {
        target[i] = s[i];
        i += 1;
    }
    target[i] = 0;
    0
}
