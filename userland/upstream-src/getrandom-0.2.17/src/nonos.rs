// NONOS backend for getrandom. The kernel exposes a CSPRNG through the CRND
// syscall (rax = the tag "CRND", rdi = buffer pointer, rsi = length), the same
// source the std platform layer draws from, so a userspace capsule fills a
// buffer with one call and no OS-specific dependency.

use crate::Error;
use core::mem::MaybeUninit;

// The CRND syscall number, "CRND" packed little-endian into the low 32 bits.
const N_CRYPTO_RANDOM: i64 = {
    let b = *b"CRND";
    (b[0] as i64) | ((b[1] as i64) << 8) | ((b[2] as i64) << 16) | ((b[3] as i64) << 24)
};

pub fn getrandom_inner(dest: &mut [MaybeUninit<u8>]) -> Result<(), Error> {
    if dest.is_empty() {
        return Ok(());
    }
    // SAFETY: the kernel writes exactly `len` bytes into `ptr`, clobbers only
    // the syscall scratch registers, and the buffer is valid for that length.
    unsafe {
        core::arch::asm!(
            "syscall",
            inout("rax") N_CRYPTO_RANDOM => _,
            in("rdi") dest.as_mut_ptr() as u64,
            in("rsi") dest.len() as u64,
            out("rcx") _,
            out("r11") _,
        );
    }
    Ok(())
}
