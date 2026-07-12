// NONOS std PAL: a blocking futex over the kernel wait queue (MFTW/MFTK). A
// waiter sleeps in the kernel instead of spinning a yield loop, and a waker
// wakes it directly, so Mutex, Condvar, RwLock, Once and thread parking pay
// a real sleep under contention rather than burning a core. The kernel caps
// each wait so a wake that races the enqueue self-heals; this side rechecks
// the atomic after every return, so the loop is correct regardless.

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

const N_MK_TIME_MILLIS: i64 = tag4(b"MTMS");
const N_MK_FUTEX_WAIT: i64 = tag4(b"MFTW");
const N_MK_FUTEX_WAKE: i64 = tag4(b"MFTK");

// Block in the kernel until the word at `addr` leaves `expected`, a waker
// signals it, or the kernel's safety cap lapses. `timeout_ms == 0` means no
// deadline (the kernel still caps each call, so the caller loops).
fn futex_wait_syscall(addr: u64, expected: u32, timeout_ms: u64) {
    // SAFETY: MFTW reads the futex word itself and only ever deschedules the
    // caller; rcx/r11 are burned by the syscall instruction.
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") N_MK_FUTEX_WAIT,
            in("rdi") addr,
            in("rsi") expected as u64,
            in("rdx") timeout_ms,
            out("rcx") _,
            out("r11") _,
        );
    }
}

// Wake up to `count` waiters on `addr` (0 means all). Returns the count woken.
fn futex_wake_syscall(addr: u64, count: u64) -> i64 {
    let r: i64;
    // SAFETY: MFTK only moves descheduled waiters back onto the run queue.
    unsafe {
        core::arch::asm!(
            "syscall",
            inout("rax") N_MK_FUTEX_WAKE => r,
            in("rdi") addr,
            in("rsi") count,
            out("rcx") _,
            out("r11") _,
        );
    }
    r
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

/// Block on `futex` while it holds `expected`, until a waker signals it or
/// the timeout lapses. Returns false only on timeout, per the futex contract.
pub fn futex_wait(futex: &Atomic<u32>, expected: u32, timeout: Option<Duration>) -> bool {
    let addr = futex as *const Atomic<u32> as u64;
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
        let remaining = match deadline {
            Some(d) => {
                let now = now_ms();
                if now >= d {
                    return false;
                }
                d - now
            }
            None => 0,
        };
        futex_wait_syscall(addr, expected, remaining);
    }
}

/// Wake one waiter blocked on `futex`. Returns whether one was woken.
#[inline]
pub fn futex_wake(futex: &Atomic<u32>) -> bool {
    let addr = futex as *const Atomic<u32> as u64;
    futex_wake_syscall(addr, 1) > 0
}

/// Wake every waiter blocked on `futex`.
#[inline]
pub fn futex_wake_all(futex: &Atomic<u32>) {
    let addr = futex as *const Atomic<u32> as u64;
    futex_wake_syscall(addr, 0);
}
