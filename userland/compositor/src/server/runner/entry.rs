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
use crate::debug;
use crate::frame_pacer;
use crate::protocol::{HDR_LEN, IPC_PAYLOAD_MAX};
use crate::state::Context;
use nonos_libc::mk_yield;

pub fn run(mut ctx: Context) -> ! {
    let mut rx = [0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut tx = [0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    debug::marker(b"server enter");
    loop {
        drain_ipc(&mut ctx, &mut rx, &mut tx);
        match frame_pacer::tick(&mut ctx) {
            Ok(()) => {}
            Err(_) if !ctx.scanout_error_reported => {
                ctx.scanout_error_reported = true;
                debug::marker(b"scanout err");
            }
            Err(_) => {}
        }
        if frame_pacer::wait_for_vsync().is_err() {
            mk_yield();
        }
    }
}
