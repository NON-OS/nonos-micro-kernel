// NONOS std PAL: randomness via the kernel crypto_random syscall.
//
// Delegates to the same kernel entropy source (rdrand/rdseed-backed) that
// the userland libc `crypto_random` uses. Raw syscall: rax = tag, rdi/rsi
// = (buf, len); the kernel fills `len` bytes and returns the count.

const fn tag4(b: &[u8; 4]) -> i64 {
    (b[0] as i64) | ((b[1] as i64) << 8) | ((b[2] as i64) << 16) | ((b[3] as i64) << 24)
}

const N_CRYPTO_RANDOM: i64 = tag4(b"CRND");

#[inline]
unsafe fn syscall2(num: i64, a1: u64, a2: u64) -> i64 {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "syscall",
            inout("rax") num => ret,
            in("rdi") a1,
            in("rsi") a2,
            out("rcx") _,
            out("r11") _,
        );
    }
    ret
}

pub fn fill_bytes(bytes: &mut [u8]) {
    if bytes.is_empty() {
        return;
    }
    unsafe {
        syscall2(N_CRYPTO_RANDOM, bytes.as_mut_ptr() as u64, bytes.len() as u64);
    }
}
