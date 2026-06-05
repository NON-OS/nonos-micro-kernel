// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use nonos_libc::mk_irq_ack;

use crate::mouse::publish_mouse;
use crate::poll::drain;
use crate::server::context::Context;
use crate::server::irq_seq::poll_seq;

pub fn pump(ctx: &mut Context, prev_buttons: &mut u8) {
    let kbd = poll_seq(ctx.driver.irq_grant_id);
    let aux = poll_seq(ctx.driver.aux_irq_grant_id);
    let fired = kbd != ctx.last_kbd_seq || aux != ctx.last_aux_seq;
    let unbound = ctx.driver.irq_grant_id == 0 && ctx.driver.aux_irq_grant_id == 0;
    if fired || unbound {
        drain(
            ctx.driver.pio_grant_id,
            &mut ctx.drainer,
            &mut ctx.ring,
            &mut ctx.mouse,
            &mut ctx.mouse_ring,
        );
        ctx.last_kbd_seq = poll_seq(ctx.driver.irq_grant_id);
        ctx.last_aux_seq = poll_seq(ctx.driver.aux_irq_grant_id);
        let _ = mk_irq_ack(ctx.driver.irq_grant_id);
        let _ = mk_irq_ack(ctx.driver.aux_irq_grant_id);
    }
    while let Some(ev) = ctx.mouse_ring.pop() {
        let _ = publish_mouse(ev, *prev_buttons);
        *prev_buttons = ev.buttons;
    }
}
