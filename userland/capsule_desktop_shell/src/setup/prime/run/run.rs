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

use crate::debug;
use crate::input_router_client;
use crate::market_client;
use crate::render::paint_chrome;
use crate::setup::prime::{overlay, peers, register};
use crate::state::{Context, SpotlightState, TrayTable};
use crate::wm_client;
use nonos_libc::mk_munmap;

const INPUT_POINTER_ABS_BIT: u32 = 1 << 3;
const INPUT_BUTTON_DOWN_BIT: u32 = 1 << 5;
const SHELL_INPUT_MASK: u32 = INPUT_POINTER_ABS_BIT | INPUT_BUTTON_DOWN_BIT;

pub fn run() -> Result<Context, &'static str> {
    let peers = peers::resolve()?;
    super::require_status::require_status(wm_client::healthcheck(peers.wm_port, 2))?;
    super::apply_wallpaper_policy::apply_wallpaper_policy(peers.wallpaper_port)?;
    super::require_status::require_status(market_client::healthcheck(peers.market_port, 4))?;
    let overlay = overlay::allocate(peers.compositor_port, 1)?;
    let mut ctx = Context {
        compositor_port: peers.compositor_port,
        width: overlay.width,
        height: overlay.height,
        stride: overlay.stride,
        backing_va: overlay.backing_va,
        tray: TrayTable::new(),
        spotlight: SpotlightState::new(),
        last_notify_level: None,
        next_request_id: 2,
    };
    paint_chrome(&ctx);
    let rid = ctx.issue_request_id();
    if let Err(e) = register::register_overlay(peers.compositor_port, rid, &overlay) {
        if mk_munmap(overlay.backing_va as *mut u8, overlay.byte_len as usize) < 0 {
            return Err("overlay munmap failed");
        }
        return Err(e);
    }
    let rid = ctx.issue_request_id();
    input_router_client::subscribe(peers.input_router_port, rid, SHELL_INPUT_MASK)?;
    debug::marker(b"[LIVE-GUI] launcher visible");
    debug::marker(b"scene submitted");
    debug::marker(b"peers ok");
    Ok(ctx)
}
