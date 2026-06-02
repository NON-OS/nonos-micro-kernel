use alloc::vec;

use nonos_libc::{mk_exit, mk_ipc_recv_from, INPUT_KIND_KEY_DOWN};

use crate::clients::{compositor, input_router};
use crate::debug;
use crate::protocol::{parse_delivery, DELIVERY_LEN};
use crate::render::screens;
use crate::state::Context;

use super::step::{self, DONE};

pub fn run(mut ctx: Context) -> ! {
    let _ = input_router::subscribe(ctx.router_port, 1);
    let _ = input_router::grab_keyboard(ctx.router_port, 2);
    debug::marker(b"subscribed+grabbed");
    redraw(&ctx);
    let mut rx = vec![0u8; DELIVERY_LEN.max(64)];
    loop {
        let mut sender = 0u32;
        let n = mk_ipc_recv_from(0, rx.as_mut_ptr(), rx.len(), 0, &mut sender);
        if n <= 0 {
            continue;
        }
        let Some(ev) = parse_delivery(&rx[..n as usize]) else {
            continue;
        };
        if ev.kind != INPUT_KIND_KEY_DOWN {
            continue;
        }
        let outcome = screens::on_key(&mut ctx, ev.code);
        ctx.step = step::apply(ctx.step, outcome);
        if ctx.step >= DONE {
            debug::marker(b"wizard done");
            mk_exit(0);
        }
        redraw(&ctx);
    }
}

fn redraw(ctx: &Context) {
    screens::draw(ctx);
    let _ = compositor::damage_commit(ctx.compositor_port, 9, ctx.width, ctx.height);
}
