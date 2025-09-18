use crate::{
    cstr::strlen,
    syscalls::{exit, write},
};

pub fn tokenize<'a>(s: &'a [u8], span: &[Span], buf: &mut [&'a [u8]]) -> usize {
    if buf.len() != span.len() {
        write(1, b"Wrong buffer size for tokenizer\nAborting...\n");
        exit(-1);
    }

    let mut i = 0;
    while i < buf.len() {
        buf[i] = &s[span[i].start..span[i].end];
        i += 1;
    }

    i
}

pub fn split(s: &mut [u8], c: u8, buf: &mut [Span]) -> usize {
    let len = s.len();
    let buf_len = buf.len();

    let mut i = 0;
    let mut j = 0;

    while i < len && j < buf_len {
        while i < len && (s[i] == c || s[i] == 0) {
            i += 1;
        }

        if i >= len {
            break;
        }

        let start = i;
        while i < len && s[i] != c && s[i] != 0 {
            i += 1;
        }
        let end = i;

        if i < len && s[i] != 0 {
            s[i] = 0;
            i += 1;
        }

        buf[j] = Span {
            start: start,
            end: end,
        };

        j += 1;
    }

    j
}

pub fn trim(s: &mut [u8]) -> Option<&mut [u8]> {
    let len = strlen(s);

    let mut i = 0;
    while i < len && is_space(s[i]) {
        i += 1;
    }
    if i == len {
        if let Some(byte) = s.first_mut() {
            *byte = 0;
        }
        return None;
    }

    let mut j = len;
    while j > i && s[j - 1] == 0 {
        j -= 1;
    }
    while j > i && is_space(s[j - 1]) {
        s[j - 1] = 0;
        j -= 1;
    }
    if j < len {
        s[j] = 0;
    }

    return Some(&mut s[i..j]);
}

#[inline(always)]
fn is_space(c: u8) -> bool {
    matches!(c, b' ' | b'\n' | b'\r' | b'\t' | 0x0B | 0x0C)
}

#[derive(Clone, Copy)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            start: start,
            end: end,
        }
    }
}
