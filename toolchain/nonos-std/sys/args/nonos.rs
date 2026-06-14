// NONOS std args: reads the process argv via the mk_args syscall and
// hands it to the shared Args iterator. argv is NUL-separated bytes.

pub use super::common::Args;

use crate::ffi::OsString;
use crate::string::String;
use crate::vec::Vec;

const fn tag4(b: &[u8; 4]) -> i64 {
    (b[0] as i64) | ((b[1] as i64) << 8) | ((b[2] as i64) << 16) | ((b[3] as i64) << 24)
}

const N_MK_ARGS: i64 = tag4(b"MKAR");

unsafe fn mk_args(buf: *mut u8, len: usize) -> i64 {
    let r: i64;
    unsafe {
        core::arch::asm!(
            "syscall",
            inout("rax") N_MK_ARGS => r,
            in("rdi") buf as u64,
            in("rsi") len as u64,
            out("rcx") _,
            out("r11") _,
        );
    }
    r
}

fn collect() -> Vec<OsString> {
    let needed = unsafe { mk_args(core::ptr::null_mut(), 0) };
    if needed <= 0 {
        return Vec::new();
    }
    let mut buf = crate::vec![0u8; needed as usize];
    let n = unsafe { mk_args(buf.as_mut_ptr(), buf.len()) };
    if n <= 0 {
        return Vec::new();
    }
    buf.truncate(n as usize);
    buf.split(|&b| b == 0).map(|s| OsString::from(String::from_utf8_lossy(s).into_owned())).collect()
}

pub fn args() -> Args {
    Args::new(collect())
}
