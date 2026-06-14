// NONOS std threads: real OS threads via the mk_thread_spawn syscall. The
// kernel creates a thread sharing the capsule's address space; we hand it an
// mmap'd stack whose top word holds the boxed start routine. A small naked
// trampoline pops that pointer into rdi and calls the Rust entry. Join spins
// on a shared flag the thread sets before it exits (single-core, cooperative).

use crate::ffi::CStr;
use crate::io;
use crate::num::NonZero;
use crate::sync::Arc;
use crate::sync::atomic::{AtomicBool, Ordering};
use crate::thread::ThreadInit;
use crate::time::Duration;

const fn tag4(b: &[u8; 4]) -> i64 {
    (b[0] as i64) | ((b[1] as i64) << 8) | ((b[2] as i64) << 16) | ((b[3] as i64) << 24)
}

const N_MK_THREAD_SPAWN: i64 = tag4(b"MTSP");
const N_MK_MMAP: i64 = tag4(b"MMAP");
const N_MK_EXIT: i64 = tag4(b"MEXT");
const N_MK_YIELD: i64 = tag4(b"MYLD");
const N_MK_TIME_MILLIS: i64 = tag4(b"MTMS");

pub const DEFAULT_MIN_STACK_SIZE: usize = 1 << 20;

unsafe fn mmap(len: usize) -> *mut u8 {
    let r: i64;
    unsafe {
        core::arch::asm!("syscall", inout("rax") N_MK_MMAP => r, in("rdi") 0u64,
            in("rsi") len as u64, in("rdx") 3u64, in("r10") 0u64, out("rcx") _, out("r11") _);
    }
    if r <= 0 { core::ptr::null_mut() } else { r as *mut u8 }
}

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

struct Start {
    init: Box<ThreadInit>,
    done: Arc<AtomicBool>,
}

#[unsafe(naked)]
unsafe extern "C" fn trampoline() -> ! {
    core::arch::naked_asm!("pop rdi", "call {f}", "ud2", f = sym thread_main);
}

extern "C" fn thread_main(arg: *mut u8) -> ! {
    let start = unsafe { Box::from_raw(arg as *mut Start) };
    let run = start.init.init();
    run();
    start.done.store(true, Ordering::Release);
    unsafe {
        core::arch::asm!("syscall", in("rax") N_MK_EXIT, in("rdi") 0u64, options(noreturn));
    }
}

pub struct Thread {
    done: Arc<AtomicBool>,
}

impl Thread {
    pub unsafe fn new(stack: usize, init: Box<ThreadInit>) -> io::Result<Thread> {
        let done = Arc::new(AtomicBool::new(false));
        let start = Box::new(Start { init, done: done.clone() });
        let raw = Box::into_raw(start);
        let size = if stack < DEFAULT_MIN_STACK_SIZE { DEFAULT_MIN_STACK_SIZE } else { stack };
        let base = unsafe { mmap(size) };
        if base.is_null() {
            drop(unsafe { Box::from_raw(raw) });
            return Err(io::Error::from(io::ErrorKind::OutOfMemory));
        }
        let top = base as usize + size;
        let sp = (top & !0xf) - 8;
        unsafe { *(sp as *mut u64) = raw as u64 };
        let tid: i64;
        unsafe {
            core::arch::asm!("syscall", inout("rax") N_MK_THREAD_SPAWN => tid,
                in("rdi") trampoline as usize as u64, in("rsi") sp as u64,
                out("rcx") _, out("r11") _);
        }
        if tid <= 0 {
            drop(unsafe { Box::from_raw(raw) });
            return Err(io::Error::from(io::ErrorKind::Other));
        }
        Ok(Thread { done })
    }

    pub fn join(self) {
        while !self.done.load(Ordering::Acquire) {
            raw_yield();
        }
    }
}

pub fn available_parallelism() -> io::Result<NonZero<usize>> {
    Ok(NonZero::new(1).unwrap())
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
