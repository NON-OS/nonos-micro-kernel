#![no_std]
#![no_main]

extern crate alloc;

mod clients;
mod debug;
mod setup;
mod state;

use nonos_libc::{heap_init, mk_exit, mk_yield};

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    debug::marker(b"probe boot");
    let _ctx = match setup::run() {
        Ok(ctx) => ctx,
        Err(_) => {
            debug::marker(b"setup failed");
            mk_exit(2);
        }
    };
    loop {
        mk_yield();
    }
}
