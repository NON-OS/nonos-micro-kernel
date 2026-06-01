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

use super::super::discover;
use crate::compositor_client::healthcheck;

pub struct Peers {
    pub compositor_port: u32,
    pub input_router_port: u32,
    pub wm_port: u32,
    pub wallpaper_port: u32,
    pub market_port: u32,
}

pub fn resolve() -> Result<Peers, &'static str> {
    let compositor_port = discover::require_compositor()?;
    healthcheck(compositor_port, 1)?;
    let input_router_port = discover::require_input_router()?;
    let wm_port = discover::require_wm()?;
    let wallpaper_port = discover::require_wallpaper()?;
    let market_port = discover::try_market();
    Ok(Peers { compositor_port, input_router_port, wm_port, wallpaper_port, market_port })
}
