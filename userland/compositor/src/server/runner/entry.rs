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

use alloc::vec;

use super::drain::drain_ipc;
use crate::debug;
use crate::frame_pacer;
use crate::protocol::{HDR_LEN, IPC_PAYLOAD_MAX};
use crate::state::Context;

pub fn run(mut ctx: Context) -> ! {
    let mut rx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut tx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    loop {
        drain_ipc(&mut ctx, &mut rx, &mut tx);
        match frame_pacer::tick(&mut ctx) {
            Ok(()) => {}
            Err(e) if !ctx.scanout_error_reported => {
                debug::marker(e.as_bytes());
                ctx.scanout_error_reported = true;
            }
            Err(_) => {}
        }
        let _ = frame_pacer::wait_for_vsync();
    }
}
