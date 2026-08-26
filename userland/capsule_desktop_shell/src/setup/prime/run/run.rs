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

use crate::render::paint_chrome;
use crate::setup::prime::{open_chrome_windows, overlay, peers};
use crate::state::Context;

pub fn run() -> Result<Context, &'static str> {
    let peers = peers::resolve()?;
    super::healthcheck_peers::healthcheck_peers(&peers)?;
    super::apply_wallpaper_policy::apply_wallpaper_policy(peers.wallpaper_port)?;
    let overlay = overlay::allocate(peers.compositor_port, 1)?;
    let mut ctx = super::build_context::build_context(&peers, &overlay);
    crate::render::ui_font::set_scale(ctx.scale);
    paint_chrome(&ctx);
    super::register_overlay::register_overlay(&mut ctx, &overlay)?;
    super::commit_overlay::commit_overlay(&mut ctx)?;
    open_chrome_windows::open_chrome_windows(&mut ctx)?;
    super::subscribe_wm::subscribe_wm(&mut ctx, peers.wm_port);
    super::subscribe_input::subscribe_input(&mut ctx, peers.input_router_port);
    Ok(ctx)
}
