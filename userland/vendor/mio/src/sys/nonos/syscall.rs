// The raw NONOS syscall entry the backend needs: a tag builder and the
// argument-count variants used to reach the net.sockets service (MSVL to
// resolve its port, MICL for a request/reply) and to cooperatively yield
// between poll rounds (MYLD). NONOS has no timed-sleep syscall, so the selector
// bounds its wait with the monotonic clock and yields the core each round.
// Shared by the selector and the net modules.

pub(super) const fn tag4(b: &[u8; 4]) -> i64 {
    (b[0] as i64) | ((b[1] as i64) << 8) | ((b[2] as i64) << 16) | ((b[3] as i64) << 24)
}

// No-argument syscall (MYLD reschedules the caller). rax carries the tag and is
// clobbered by the return value; rcx/r11 are burned by the syscall instruction.
pub(super) unsafe fn sys0(num: i64) {
    core::arch::asm!("syscall", inout("rax") num => _, out("rcx") _, out("r11") _);
}

// SAFETY (shared by all three wrappers): each issues the NONOS `syscall`
// instruction with the tag in rax and the documented argument registers,
// clobbering only rcx/r11 as the ABI requires. Callers pass valid pointer and
// length pairs for the buffers a given tag reads or writes; no wrapper retains
// a borrow past the call.
pub(super) unsafe fn sys5(num: i64, a1: u64, a2: u64, a3: u64, a4: u64, a5: u64) -> i64 {
    let r: i64;
    core::arch::asm!("syscall", inout("rax") num => r, in("rdi") a1, in("rsi") a2,
        in("rdx") a3, in("r10") a4, in("r8") a5, out("rcx") _, out("r11") _);
    r
}

pub(super) unsafe fn sys6(num: i64, a1: u64, a2: u64, a3: u64, a4: u64, a5: u64, a6: u64) -> i64 {
    let r: i64;
    core::arch::asm!("syscall", inout("rax") num => r, in("rdi") a1, in("rsi") a2,
        in("rdx") a3, in("r10") a4, in("r8") a5, in("r9") a6, out("rcx") _, out("r11") _);
    r
}
