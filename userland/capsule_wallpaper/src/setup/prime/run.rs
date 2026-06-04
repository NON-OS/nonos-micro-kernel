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
use crate::catalog_client::lookup_catalog;
use crate::compositor_client::healthcheck;
use crate::paint::{decode_jpeg, fill_argb, paint_image};
use crate::policy_client::lookup_policy;
use crate::state::{Context, FadeTimeline, Policy};

const DEFAULT_ARGB: u32 = 0xFF00_80FF;
const EMBEDDED_WALLPAPER: &[u8] =
    include_bytes!("../../../../../nonos-data/wallpapers/special-variant-12.jpg");

pub fn run() -> Result<Context, &'static str> {
    let compositor_port = discover::lookup_compositor_port()?;
    healthcheck(compositor_port, 1)?;
    let backing = backing::allocate(compositor_port, 2)?;
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
        next_request_id: 3,
        policy_port: lookup_policy(),
        catalog_port: lookup_catalog(),
        applied_wallpaper: None,
        subscriber_ticks: 0,
    };
    ctx.set_argb(DEFAULT_ARGB);
    if let Some(img) = decode_jpeg(EMBEDDED_WALLPAPER) {
        paint_image(&ctx, &img);
    }
    let rid = ctx.issue_request_id();
    register::register_wallpaper(compositor_port, rid, &backing)?;
    Ok(ctx)
}
