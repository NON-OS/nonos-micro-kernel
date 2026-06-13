// NONOS std PAL: global allocator.
//
// Phase 1: a self-contained bump arena in BSS so `std` has a working
// GlobalAlloc with no external dependency and no failure path. Memory is
// never reclaimed yet; a later phase swaps this for an mmap-backed heap
// with free. Freshly bumped bytes are zero (BSS), so alloc_zeroed is free.

use crate::alloc::{GlobalAlloc, Layout, System};
use crate::sync::atomic::{AtomicUsize, Ordering};

const ARENA_SIZE: usize = 16 * 1024 * 1024;

#[repr(C, align(4096))]
struct Arena([u8; ARENA_SIZE]);

static mut ARENA: Arena = Arena([0u8; ARENA_SIZE]);
static OFFSET: AtomicUsize = AtomicUsize::new(0);

#[stable(feature = "alloc_system_type", since = "1.28.0")]
unsafe impl GlobalAlloc for System {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let base = core::ptr::addr_of_mut!(ARENA) as usize;
        let align = layout.align();
        loop {
            let cur = OFFSET.load(Ordering::Relaxed);
            let start = (base + cur + align - 1) & !(align - 1);
            let end = match start.checked_add(layout.size()) {
                Some(e) => e,
                None => return core::ptr::null_mut(),
            };
            if end > base + ARENA_SIZE {
                return core::ptr::null_mut();
            }
            if OFFSET
                .compare_exchange_weak(cur, end - base, Ordering::AcqRel, Ordering::Relaxed)
                .is_ok()
            {
                return start as *mut u8;
            }
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}
