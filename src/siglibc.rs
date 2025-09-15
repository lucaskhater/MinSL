#[repr(C)]
pub struct siginfo_t {
    pub si_signo: i32,
    pub si_errno: i32,
    pub si_code: i32,
    pub __pad0: i32,
    pub _sifields: [u8; 112],
}
