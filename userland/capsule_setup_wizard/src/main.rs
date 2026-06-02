#![no_std]
#![no_main]

extern crate alloc;

mod debug;

use nonos_libc::{heap_init, mk_exit};

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    debug::marker(b"wizard boot");
    mk_exit(0)
}
