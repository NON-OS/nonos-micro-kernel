use core::arch::asm;

pub const fn tag4(s: &[u8; 4]) -> u64 {
    (s[0] as u64) | (s[1] as u64) << 8 | (s[2] as u64) << 16 | (s[3] as u64) << 24
}

pub const MK_DEBUG: u64 = tag4(b"MDBG");
pub const MK_EXIT: u64 = tag4(b"MEXT");
pub const MK_GETPID: u64 = tag4(b"MGPD");
pub const MK_MMAP: u64 = tag4(b"MMAP");
pub const MK_MUNMAP: u64 = tag4(b"MUMP");
pub const MK_TIME_MILLIS: u64 = tag4(b"MTMS");
pub const MK_YIELD: u64 = tag4(b"MYLD");
pub const MK_CRYPTO_RANDOM: u64 = tag4(b"CRND");

#[inline]
pub unsafe fn syscall0(n: u64) -> i64 {
    let r: i64;
    unsafe {
        asm!("syscall", inlateout("rax") n => r,
             lateout("rdi") _, lateout("rsi") _, lateout("rdx") _,
             lateout("rcx") _, lateout("r11") _, options(nostack));
    }
    r
}

#[inline]
pub unsafe fn syscall1(n: u64, a0: u64) -> i64 {
    let r: i64;
    unsafe {
        asm!("syscall", inlateout("rax") n => r, in("rdi") a0,
             lateout("rsi") _, lateout("rdx") _,
             lateout("rcx") _, lateout("r11") _, options(nostack));
    }
    r
}

#[inline]
pub unsafe fn syscall2(n: u64, a0: u64, a1: u64) -> i64 {
    let r: i64;
    unsafe {
        asm!("syscall", inlateout("rax") n => r, in("rdi") a0, in("rsi") a1,
             lateout("rdx") _,
             lateout("rcx") _, lateout("r11") _, options(nostack));
    }
    r
}

#[inline]
pub unsafe fn syscall3(n: u64, a0: u64, a1: u64, a2: u64) -> i64 {
    let r: i64;
    unsafe {
        asm!("syscall", inlateout("rax") n => r, in("rdi") a0, in("rsi") a1,
             in("rdx") a2, lateout("rcx") _, lateout("r11") _, options(nostack));
    }
    r
}

#[inline]
pub unsafe fn syscall4(n: u64, a0: u64, a1: u64, a2: u64, a3: u64) -> i64 {
    let r: i64;
    unsafe {
        asm!("syscall", inlateout("rax") n => r, in("rdi") a0, in("rsi") a1,
             in("rdx") a2, in("r10") a3,
             lateout("rcx") _, lateout("r11") _, options(nostack));
    }
    r
}

#[inline]
pub unsafe fn syscall5(n: u64, a0: u64, a1: u64, a2: u64, a3: u64, a4: u64) -> i64 {
    let r: i64;
    unsafe {
        asm!("syscall", inlateout("rax") n => r, in("rdi") a0, in("rsi") a1,
             in("rdx") a2, in("r10") a3, in("r8") a4,
             lateout("rcx") _, lateout("r11") _, options(nostack));
    }
    r
}

#[inline]
pub unsafe fn syscall6(n: u64, a0: u64, a1: u64, a2: u64, a3: u64, a4: u64, a5: u64) -> i64 {
    let r: i64;
    unsafe {
        asm!("syscall", inlateout("rax") n => r, in("rdi") a0, in("rsi") a1,
             in("rdx") a2, in("r10") a3, in("r8") a4, in("r9") a5,
             lateout("rcx") _, lateout("r11") _, options(nostack));
    }
    r
}
