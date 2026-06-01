use alloc::vec;

use nonos_libc::{mk_ipc_recv_from, InputEvent, INPUT_KIND_KEY_DOWN};

use crate::protocol::{parse_delivery, DELIVERY_LEN};
use crate::state::Context;
use crate::{clients, debug};

const SERVICE_INBOX: u64 = 0;
const RECV_BLOCK_FOREVER: u64 = 0;

pub fn run(mut ctx: Context) -> ! {
    if clients::input_router::subscribe(ctx.router_port, 1).is_err() {
        debug::marker(b"subscribe err");
    }
    if clients::input_router::grab_keyboard(ctx.router_port, 2).is_err() {
        debug::marker(b"grab err");
    }
    debug::marker(b"subscribed+grabbed");
    let mut rx = vec![0u8; DELIVERY_LEN.max(64)];
    loop {
        let mut sender = 0u32;
        let n = mk_ipc_recv_from(
            SERVICE_INBOX,
            rx.as_mut_ptr(),
            rx.len(),
            RECV_BLOCK_FOREVER,
            &mut sender,
        );
        if n <= 0 {
            continue;
        }
        let Some(ev) = parse_delivery(&rx[..n as usize]) else {
            continue;
        };
        if ev.kind == INPUT_KIND_KEY_DOWN {
            on_key(&mut ctx, ev);
        }
    }
}

fn on_key(_ctx: &mut Context, ev: InputEvent) {
    debug::marker(b"[KEY]");
    debug::marker_u32(ev.code);
}
