#![no_std]
#![no_main]

extern crate alloc;

mod clients;
mod debug;
mod protocol;
mod render;
mod server;
mod setup;
mod state;

use nonos_libc::{heap_init, mk_exit};

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    debug::marker(b"probe boot");
    let ctx = match setup::run() {
        Ok(ctx) => ctx,
        Err(e) => {
            debug::marker(e.as_bytes());
            mk_exit(2);
        }
    };
    server::runner::run(ctx);
}
