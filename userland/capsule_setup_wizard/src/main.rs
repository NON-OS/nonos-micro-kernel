#![no_std]
#![no_main]

extern crate alloc;

mod clients;
mod debug;
mod protocol;
mod setup;
mod state;

use nonos_libc::{heap_init, mk_exit, mk_ipc_recv_from};

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    debug::marker(b"wizard boot");
    let ctx = match setup::run() {
        Ok(ctx) => ctx,
        Err(e) => {
            debug::marker(e.as_bytes());
            mk_exit(2);
        }
    };
    let _ = clients::input_router::subscribe(ctx.router_port, 1);
    let _ = clients::input_router::grab_keyboard(ctx.router_port, 2);
    debug::marker(b"subscribed+grabbed");
    let mut rx = alloc::vec![0u8; 64];
    let mut sender = 0u32;
    let _ = mk_ipc_recv_from(0, rx.as_mut_ptr(), rx.len(), 0, &mut sender);
    debug::marker(b"first key -- exit");
    let _ = ctx;
    mk_exit(0)
}
