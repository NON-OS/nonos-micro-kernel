#![no_std]
#![no_main]

extern crate alloc;

mod clients;
mod consent;
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
    let ctx = match setup::run() {
        Ok(ctx) => ctx,
        Err(_) => mk_exit(2),
    };
    let mut ctx = ctx;
    server::restore_poll::poll(&mut ctx);
    server::runner::run(ctx)
}
