// NONOS std threads. Capsules are single-threaded: thread_local is the
// no_threads backend, so a second thread shares the main thread's TLS
// statics and std aborts in the child on set_current. Until the kernel
// carries a per-thread fs base across MTSP threads, spawn fails in the
// parent with Unsupported. Yield and sleep are real syscalls.

use crate::ffi::CStr;
use crate::io;
use crate::num::NonZero;
use crate::thread::ThreadInit;
use crate::time::Duration;

const fn tag4(b: &[u8; 4]) -> i64 {
    (b[0] as i64) | ((b[1] as i64) << 8) | ((b[2] as i64) << 16) | ((b[3] as i64) << 24)
}

const N_MK_YIELD: i64 = tag4(b"MYLD");
const N_MK_TIME_MILLIS: i64 = tag4(b"MTMS");

pub const DEFAULT_MIN_STACK_SIZE: usize = 1 << 20;

fn raw_yield() {
    unsafe {
        core::arch::asm!("syscall", in("rax") N_MK_YIELD, out("rcx") _, out("r11") _);
    }
}

fn now_ms() -> u64 {
    let r: i64;
    unsafe {
        core::arch::asm!("syscall", inout("rax") N_MK_TIME_MILLIS => r, out("rcx") _, out("r11") _);
    }
    if r < 0 { 0 } else { r as u64 }
}

pub struct Thread(!);

impl Thread {
    pub unsafe fn new(_stack: usize, _init: Box<ThreadInit>) -> io::Result<Thread> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "NONOS capsules are single-threaded; thread spawn is not supported",
        ))
    }

    pub fn join(self) {
        self.0
    }
}

pub fn available_parallelism() -> io::Result<NonZero<usize>> {
    Ok(NonZero::<usize>::MIN)
}

pub fn current_os_id() -> Option<u64> {
    None
}

pub fn yield_now() {
    raw_yield();
}

pub fn set_name(_name: &CStr) {}

pub fn sleep(dur: Duration) {
    let ms = dur.as_millis() as u64;
    let start = now_ms();
    while now_ms().wrapping_sub(start) < ms {
        raw_yield();
    }
}
