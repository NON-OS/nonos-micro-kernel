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

use nonos_libc::mk_munmap;

use crate::setup::prime::{close_chrome_windows, overlay::Overlay, register};
use crate::state::Context;

pub fn register_overlay(ctx: &mut Context, overlay: &Overlay) -> Result<(), &'static str> {
    let rid = ctx.issue_request_id();
    if let Err(e) = register::register_overlay(ctx.compositor_port, rid, overlay) {
        if e == "compositor call failed" {
            return Ok(());
        }
        close_chrome_windows::close_chrome_windows(ctx);
        if mk_munmap(overlay.backing_va as *mut u8, overlay.byte_len as usize) < 0 {
            return Err("overlay munmap failed");
        }
        return Err(e);
    }
    Ok(())
}
