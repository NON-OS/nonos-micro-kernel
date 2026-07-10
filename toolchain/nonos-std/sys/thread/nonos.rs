// NONOS std threads over the MTSP syscall. The kernel clones the current
// capsule's address space handle into a fresh schedulable task, so a thread
// here is a task sharing memory and capabilities with its spawner. The
// spawn ABI carries only an entry point and a stack pointer; the ThreadInit
// pointer rides the new stack, and a naked trampoline moves it into rdi
// before entering Rust. Join polls MPAL (pid-alive) with a yield between
// probes; the kernel reaps a dead task from the process table within a
// timer tick, after which its user stack is provably untouched and can be
// freed. Detached threads leak their stack, since nobody learns when the
// task dies.

use crate::alloc::{GlobalAlloc, Layout, System};
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
const N_MK_THREAD_SPAWN: i64 = tag4(b"MTSP");
const N_MK_PID_ALIVE: i64 = tag4(b"MPAL");
const N_MK_EXIT: i64 = tag4(b"MEXT");
const N_MK_GETPID: i64 = tag4(b"MGPD");

pub const DEFAULT_MIN_STACK_SIZE: usize = 1 << 20;

fn raw_yield() {
    // SAFETY: MYLD takes no arguments; rcx/r11 are burned by syscall.
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

fn pid_alive(tid: u32) -> bool {
    let r: i64;
    // SAFETY: MPAL takes a pid in rdi and returns 0/1 in rax.
    unsafe {
        core::arch::asm!("syscall", inout("rax") N_MK_PID_ALIVE => r, in("rdi") tid as u64,
            out("rcx") _, out("r11") _);
    }
    r == 1
}

fn thread_spawn(entry: u64, stack: u64) -> i64 {
    let r: i64;
    // SAFETY: MTSP takes (entry, stack) in (rdi, rsi) and returns the new
    // tid or a negative errno in rax.
    unsafe {
        core::arch::asm!("syscall", inout("rax") N_MK_THREAD_SPAWN => r, in("rdi") entry,
            in("rsi") stack, out("rcx") _, out("r11") _);
    }
    r
}

pub struct Thread {
    tid: u32,
    stack: *mut u8,
    stack_layout: Layout,
}

// SAFETY: the raw stack pointer is only dereferenced by the spawned thread;
// the handle just owns the allocation for the post-join free.
unsafe impl Send for Thread {}
// SAFETY: same reasoning; the handle itself never aliases the stack memory.
unsafe impl Sync for Thread {}

// The kernel enters the new task exactly at `entry` with rsp set to the
// value passed to MTSP. The parent leaves the ThreadInit pointer just above
// that rsp, so the first real instruction lifts it into the C argument
// register before calling into Rust. rsp is 16-aligned at the call site,
// which is what the SysV ABI expects.
#[unsafe(naked)]
unsafe extern "C" fn thread_entry() -> ! {
    core::arch::naked_asm!(
        "mov rdi, [rsp + 8]",
        "call {start}",
        "ud2",
        start = sym thread_start,
    )
}

extern "C" fn thread_start(data: *mut ThreadInit) -> ! {
    // TLS first: everything below (current-thread setup, dtors) needs it.
    crate::sys::thread_local::key::init_tls();
    run_init(data);
    // SAFETY: init_tls ran on this thread and no thread-local access
    // happens after this call.
    unsafe { crate::sys::thread_local::key::destroy_tls() };
    // SAFETY: MEXT never returns; the kernel reaps this task.
    unsafe {
        core::arch::asm!("syscall", in("rax") N_MK_EXIT, in("rdi") 0u64, options(noreturn));
    }
}

// Kept out of thread_start so no stack slot holding `init` state outlives
// the TLS teardown (the same reordering hazard the xous port documents).
#[inline(never)]
fn run_init(data: *mut ThreadInit) {
    // SAFETY: recreates the box leaked by `Thread::new`; the kernel spawn
    // succeeded, so this thread is the sole owner.
    let init = unsafe { Box::from_raw(data) };
    let rust_start = init.init();
    rust_start();
}

impl Thread {
    pub unsafe fn new(stack: usize, init: Box<ThreadInit>) -> io::Result<Thread> {
        // Sixteen-byte alignment for the ABI, and room above the entry rsp
        // for the ThreadInit pointer slot.
        let size = stack.max(DEFAULT_MIN_STACK_SIZE);
        let size = size.checked_add(15).map(|s| s & !15).unwrap_or(size);
        let layout = Layout::from_size_align(size, 16)
            .map_err(|_| io::const_error!(io::ErrorKind::InvalidInput, "bad stack size"))?;
        // SAFETY: layout is non-zero and valid.
        let base = unsafe { System.alloc(layout) };
        if base.is_null() {
            return Err(io::const_error!(io::ErrorKind::OutOfMemory, "thread stack allocation"));
        }
        let data = Box::into_raw(init);
        let top = base as u64 + size as u64;
        let entry_rsp = top - 16;
        // SAFETY: top-8..top lies inside the fresh allocation; the spawned
        // thread reads it exactly once from [rsp + 8].
        unsafe { ((top - 8) as *mut u64).write((data as usize) as u64) };
        let tid = thread_spawn(thread_entry as usize as u64, entry_rsp);
        if tid <= 0 {
            // The kernel refused the spawn, so the box was never consumed
            // and this thread still owns both allocations.
            unsafe {
                drop(Box::from_raw(data));
                System.dealloc(base, layout);
            }
            return Err(io::const_error!(io::ErrorKind::Uncategorized, "thread spawn rejected"));
        }
        Ok(Thread { tid: tid as u32, stack: base, stack_layout: layout })
    }

    pub fn join(self) {
        while pid_alive(self.tid) {
            raw_yield();
        }
        // The task is gone from the process table, so nothing can execute
        // on this stack anymore.
        // SAFETY: `stack` came from System.alloc with `stack_layout` and is
        // freed exactly once; `self` is consumed here, skipping Drop's leak.
        unsafe { System.dealloc(self.stack, self.stack_layout) };
        crate::mem::forget(self);
    }
}

impl Drop for Thread {
    fn drop(&mut self) {
        // Detached thread: its lifetime is unknowable without a wait
        // primitive, so the stack allocation is intentionally leaked.
    }
}

pub fn available_parallelism() -> io::Result<NonZero<usize>> {
    // The scheduler runs tasks across cores, but there is no syscall
    // reporting the core count yet; one is the honest lower bound.
    Ok(NonZero::<usize>::MIN)
}

pub fn current_os_id() -> Option<u64> {
    let r: i64;
    // SAFETY: MGPD takes no arguments and returns the calling task id.
    unsafe {
        core::arch::asm!("syscall", inout("rax") N_MK_GETPID => r, out("rcx") _, out("r11") _);
    }
    if r < 0 {
        None
    } else {
        Some(r as u64)
    }
}

pub fn yield_now() {
    raw_yield();
}

pub fn set_name(_name: &CStr) {}

pub fn sleep(dur: Duration) {
    let ms = dur.as_millis().min(u64::MAX as u128) as u64;
    let start = now_ms();
    while now_ms().wrapping_sub(start) < ms {
        raw_yield();
    }
}
