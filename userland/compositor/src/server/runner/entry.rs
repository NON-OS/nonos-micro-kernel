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

use super::drain::drain_ipc;
use crate::frame_pacer;
use crate::protocol::{HDR_LEN, IPC_PAYLOAD_MAX};
use crate::state::Context;
use nonos_libc::mk_yield;

// Force a full recomposite this often. Damage tracking only repaints the
// rectangles a client reports; a region that was left stale by a transient
// (a scanout size settling during boot, a client that damaged too little)
// would otherwise never heal. A periodic full pass rebuilds the whole scene
// from current surface content, so any such staleness clears within about a
// second at 60 Hz while normal frames keep the cheap partial-damage path.
const HEAL_INTERVAL_FRAMES: u32 = 45;

pub fn run(mut ctx: Context) -> ! {
    let mut rx = [0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut tx = [0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut frame: u32 = 0;
    loop {
        drain_ipc(&mut ctx, &mut rx, &mut tx);
        frame = frame.wrapping_add(1);
        if frame % HEAL_INTERVAL_FRAMES == 0 {
            ctx.damage.mark_full(ctx.width, ctx.height);
        }
        match frame_pacer::tick(&mut ctx) {
            Ok(()) => {}
            Err(_) if !ctx.scanout_error_reported => {
                ctx.scanout_error_reported = true;
            }
            Err(_) => {}
        }
        if frame_pacer::wait_for_vsync().is_err() {
            mk_yield();
        }
    }
}
