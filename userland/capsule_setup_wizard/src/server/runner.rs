use alloc::vec;

use nonos_libc::{mk_exit, mk_ipc_recv_from, INPUT_KIND_KEY_DOWN};

use crate::clients::{compositor, input_router};
use crate::protocol::{parse_delivery, DELIVERY_LEN};
use crate::render::screens;
use crate::state::Context;

use super::say::say;
use super::step::{self, DONE};

/// How long to wait for input before asking for the keyboard again.
const GRAB_RETRY_MS: u64 = 100;

/// How often to ask whether the disk has loaded consent from an earlier boot.
const RESTORE_RETRY_MS: u64 = 500;

pub fn run(mut ctx: Context) -> ! {
    if input_router::subscribe(ctx.router_port, 1).is_err() {
        say(b"[SETUP] the input router refused the subscription\n");
    }
    /*
     * The boot splash holds the keyboard until it hands off, and it may not
     * have when setup first asks. Keys sent while nobody holds it go to focus,
     * and before the desktop exists there is no focus to take them, so setup
     * asks again until it holds the keyboard.
     */
    let mut held = false;
    let mut rid = 2u32;
    redraw(&ctx);
    let mut rx = vec![0u8; DELIVERY_LEN.max(64)];
    loop {
        if !held {
            held = input_router::grab_keyboard(ctx.router_port, rid).is_ok();
            rid = rid.wrapping_add(1).max(2);
            if held {
                say(b"[SETUP] keyboard held\n");
            }
        }
        let wait = match (held, ctx.local_pending) {
            (false, _) => GRAB_RETRY_MS,
            (true, true) => RESTORE_RETRY_MS,
            (true, false) => 0,
        };
        let mut sender = 0u32;
        let n = mk_ipc_recv_from(0, rx.as_mut_ptr(), rx.len(), wait, &mut sender);
        if ctx.local_pending && super::restore_poll::poll(&mut ctx) {
            redraw(&ctx);
        }
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
            let _ = compositor::push_scene_remove(ctx.compositor_port, 3);
            mk_exit(0);
        }
        redraw(&ctx);
    }
}

fn redraw(ctx: &Context) {
    screens::draw(ctx);
    let _ = compositor::damage_commit(ctx.compositor_port, 9, ctx.width, ctx.height);
}
