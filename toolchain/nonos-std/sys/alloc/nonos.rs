// NONOS std PAL: global allocator.
//
// A real heap: dlmalloc (the same allocator std uses for wasm/sgx/xous)
// backed by NONOS page allocation through the MMAP syscall. dlmalloc owns
// free lists and coalescing, so memory is reclaimed and reused. Capsules
// are single-threaded; the lock is an uncontended spin for Sync.

use crate::alloc::{GlobalAlloc, Layout, System};
use crate::cell::SyncUnsafeCell;
use crate::ptr;
use crate::sync::atomic::{AtomicBool, Ordering};

const fn tag4(b: &[u8; 4]) -> i64 {
    (b[0] as i64) | ((b[1] as i64) << 8) | ((b[2] as i64) << 16) | ((b[3] as i64) << 24)
}

const N_MK_MMAP: i64 = tag4(b"MMAP");

unsafe fn mmap(len: usize) -> *mut u8 {
    let r: i64;
    unsafe {
        core::arch::asm!(
            "syscall",
            inout("rax") N_MK_MMAP => r,
            in("rdi") 0u64,
            in("rsi") len as u64,
            in("rdx") 3u64,
            in("r10") 0u64,
            out("rcx") _,
            out("r11") _,
        );
    }
    if r <= 0 { ptr::null_mut() } else { r as *mut u8 }
}

struct Nonos;

unsafe impl dlmalloc::Allocator for Nonos {
    fn alloc(&self, size: usize) -> (*mut u8, usize, u32) {
        let p = unsafe { mmap(size) };
        if p.is_null() { (ptr::null_mut(), 0, 0) } else { (p, size, 0) }
    }
    fn remap(&self, _ptr: *mut u8, _old: usize, _new: usize, _can_move: bool) -> *mut u8 {
        ptr::null_mut()
    }
    fn free_part(&self, _ptr: *mut u8, _old: usize, _new: usize) -> bool {
        false
    }
    fn free(&self, _ptr: *mut u8, _size: usize) -> bool {
        false
    }
    fn can_release_part(&self, _flags: u32) -> bool {
        false
    }
    fn allocates_zeros(&self) -> bool {
        false
    }
    fn page_size(&self) -> usize {
        0x1000
    }
}

struct Heap(dlmalloc::Dlmalloc<Nonos>);
unsafe impl Sync for Heap {}

static DLMALLOC: SyncUnsafeCell<Heap> =
    SyncUnsafeCell::new(Heap(dlmalloc::Dlmalloc::new_with_allocator(Nonos)));
static LOCK: AtomicBool = AtomicBool::new(false);

struct Guard;
impl Drop for Guard {
    fn drop(&mut self) {
        LOCK.store(false, Ordering::Release);
    }
}

fn lock() -> Guard {
    while LOCK.swap(true, Ordering::Acquire) {
        core::hint::spin_loop();
    }
    Guard
}

#[stable(feature = "alloc_system_type", since = "1.28.0")]
unsafe impl GlobalAlloc for System {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let _g = lock();
        unsafe { (*DLMALLOC.get()).0.malloc(layout.size(), layout.align()) }
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let _g = lock();
        unsafe { (*DLMALLOC.get()).0.calloc(layout.size(), layout.align()) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let _g = lock();
        unsafe { (*DLMALLOC.get()).0.free(ptr, layout.size(), layout.align()) }
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let _g = lock();
        unsafe { (*DLMALLOC.get()).0.realloc(ptr, layout.size(), layout.align(), new_size) }
    }
}
