use crate::cstr::*;

pub unsafe fn getenv<'a>(envp: &[&'a [u8]], s: &[u8]) -> Option<&'a [u8]> {
    if s.is_empty() || s[0] == 0 || envp[0].is_empty() {
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

pub fn capture<'a>(rsp: *const usize, env: &'a mut Env) -> (usize, &'a [&'a [u8]], &'a [&'a [u8]]) {
    unsafe {
        _capture(rsp, env);
    }

    (env.argc, env.argv, env.envp)
}

pub unsafe fn _capture<'a>(rsp: *const usize, env: &'a mut Env) {
    let argc: usize;
    let argv: *const *const u8;
    let envp: *const *const u8;

    unsafe {
        argc = *rsp;
        argv = rsp.add(1) as *const *const u8;
        envp = argv.add(argc + 1) as *const *const u8;
    }
    env.argc = argc;

    let mut i = 0;
    while i + 1 < env.argv.len() {
        let p = *argv.add(i);
        if p.is_null() {
            break;
        }
        let len = ptrlen(p);
        env.argv[i] = core::slice::from_raw_parts(p, len);
        i += 1;
    }
    env.argv[i] = &[];

    let mut j = 0;
    while j + 1 < env.envp.len() {
        let p = *envp.add(j);
        if p.is_null() {
            break;
        }
        let len = ptrlen(p);
        env.envp[j] = core::slice::from_raw_parts(p, len);
        j += 1;
    }
    env.envp[j] = &[];
}

#[inline]
unsafe fn ptrlen(mut p: *const u8) -> usize {
    let mut n = 0;
    while *p != 0 {
        p = p.add(1);
        n += 1;
    }
    n + 1
}

pub struct Env<'a> {
    pub argc: usize,
    pub argv: &'a mut [&'a [u8]],
    pub envp: &'a mut [&'a [u8]],
}
