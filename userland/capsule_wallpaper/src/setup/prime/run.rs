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
use super::{backing, register};
use crate::compositor_client::healthcheck;
use crate::paint::fill_argb;
use crate::state::{Context, FadeTimeline, Policy};

const DEFAULT_ARGB: u32 = 0xFF0A_0F0A;

pub fn run() -> Result<Context, &'static str> {
    let compositor_port = discover::lookup_compositor_port()?;
    healthcheck(compositor_port, 1)?;
    let backing = backing::allocate()?;
    fill_argb(backing.backing_va, backing.stride, backing.width, backing.height, DEFAULT_ARGB);
    let mut ctx = Context {
        compositor_port,
        width: backing.width,
        height: backing.height,
        stride: backing.stride,
        backing_va: backing.backing_va,
        argb: DEFAULT_ARGB,
        alpha: 0xFF,
        policy: Policy::Fill,
        fade: FadeTimeline::new(),
        next_request_id: 1,
    };
    ctx.set_argb(DEFAULT_ARGB);
    let rid = ctx.issue_request_id();
    register::register_wallpaper(compositor_port, rid, &backing)?;
    Ok(ctx)
}
