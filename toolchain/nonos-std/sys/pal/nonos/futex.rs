// NONOS std PAL: a polling futex. The kernel has no blocking wait queue
// syscall yet, so a waiter re-reads the atomic and yields the CPU between
// polls instead of sleeping; wakers rely on the waiter observing the store
// directly, which the futex contract allows (spurious wakeups are legal).
// Every std sync primitive built on this (Mutex, Condvar, RwLock, Once,
// thread parking) stays correct; contention costs yield loops rather than
// a true sleep until the kernel grows a wait queue.

use crate::sync::atomic::Atomic;
use crate::sync::atomic::Ordering::Acquire;
use crate::time::Duration;

/// An atomic for use as a futex that is at least 32-bits but may be larger.
pub type Futex = Atomic<Primitive>;
/// Must be the underlying type of Futex.
pub type Primitive = u32;

/// An atomic for use as a futex that is at least 8-bits but may be larger.
pub type SmallFutex = Atomic<SmallPrimitive>;
/// Must be the underlying type of SmallFutex.
pub type SmallPrimitive = u32;

const fn tag4(b: &[u8; 4]) -> i64 {
    (b[0] as i64) | ((b[1] as i64) << 8) | ((b[2] as i64) << 16) | ((b[3] as i64) << 24)
}

const N_MK_YIELD: i64 = tag4(b"MYLD");
const N_MK_TIME_MILLIS: i64 = tag4(b"MTMS");

fn raw_yield() {
    // SAFETY: MYLD takes no arguments and only tells the scheduler to run
    // someone else; rcx/r11 are the registers the syscall instruction burns.
    unsafe {
        core::arch::asm!("syscall", in("rax") N_MK_YIELD, out("rcx") _, out("r11") _);
    }
}

fn now_ms() -> u64 {
    let r: i64;
    // SAFETY: MTMS takes no arguments and returns the kernel clock in rax.
    unsafe {
        core::arch::asm!("syscall", inout("rax") N_MK_TIME_MILLIS => r, out("rcx") _, out("r11") _);
    }
    if r < 0 {
        0
    } else {
        r as u64
    }
}

/// Poll `futex` until it no longer holds `expected` or the timeout lapses.
/// Returns false only on timeout, mirroring the blocking futex contract.
pub fn futex_wait(futex: &Atomic<u32>, expected: u32, timeout: Option<Duration>) -> bool {
    let deadline = timeout.map(|dur| {
        // The kernel clock is millisecond-grained; round sub-millisecond
        // waits up so a nonzero timeout never degenerates to zero.
        let ms = dur.as_millis();
        let ms = if ms == 0 && !dur.is_zero() { 1 } else { ms.min(u64::MAX as u128) as u64 };
        now_ms().saturating_add(ms.max(1))
    });
    loop {
        if futex.load(Acquire) != expected {
            return true;
        }
        if let Some(deadline) = deadline {
            if now_ms() >= deadline {
                return false;
            }
        }
        raw_yield();
    }
}

/// Wakes are observation-based: waiters poll the atomic, so there is no
/// queue to signal. Report "nothing woken" so callers fall through to
/// their broadest wake path; the extra wakes are no-ops here.
#[inline]
pub fn futex_wake(_futex: &Atomic<u32>) -> bool {
    false
}

#[inline]
pub fn futex_wake_all(_futex: &Atomic<u32>) {}
