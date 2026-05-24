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

use super::{overlay, peers, register};
use crate::market_client;
use crate::render::paint_chrome;
use crate::state::{Context, SpotlightState, TrayTable};
use crate::wallpaper_client;
use crate::wm_client;

pub fn run() -> Result<Context, &'static str> {
    let peers = peers::resolve()?;
    let overlay = overlay::allocate()?;
    let mut ctx = Context {
        compositor_port: peers.compositor_port,
        wm_port: peers.wm_port,
        wallpaper_port: peers.wallpaper_port,
        market_port: peers.market_port,
        width: overlay.width,
        height: overlay.height,
        stride: overlay.stride,
        backing_va: overlay.backing_va,
        tray: TrayTable::new(),
        spotlight: SpotlightState::new(),
        last_notify_level: None,
        next_request_id: 1,
    };
    paint_chrome(&ctx);
    let rid = ctx.issue_request_id();
    register::register_overlay(peers.compositor_port, rid, &overlay)?;
    let _ = wm_client::healthcheck(ctx.wm_port, ctx.issue_request_id());
    let _ = wallpaper_client::set_policy(ctx.wallpaper_port, ctx.issue_request_id(), 0);
    let _ = market_client::healthcheck(ctx.market_port, ctx.issue_request_id());
    Ok(ctx)
}
