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
use nonos_app_skeleton::PaintBuffer;

use super::rail_band::{visible, Band};
use super::rail_text::RAIL_GAP;
use super::{rail_disk, rail_net, rail_sys};
use crate::rail::Rail;
use crate::term::theme::types::Theme;

/// The telemetry sections, laid out top to bottom under the navigation lists.
/// Each step advances by the section's own `height()`, the same figure the
/// scroll offset is clamped against, and a section scrolled clear of the rail
/// is skipped rather than rasterized into pixels the sub-buffer would drop.
pub fn draw(fb: &mut PaintBuffer, x: u32, y: i32, w: u32, vh: u32, rail: &Rail, t: &Theme) {
    let gap = RAIL_GAP as i32;
    let mut y = y;
    if visible(&Band { x, y, w, h: rail_sys::height() }, vh) {
        rail_sys::draw(fb, x, y, w, rail, t);
    }
    y += rail_sys::height() as i32 + gap;
    if visible(&Band { x, y, w, h: rail_net::height() }, vh) {
        rail_net::draw(fb, x, y, w, &rail.sample.net, t);
    }
    y += rail_net::height() as i32 + gap;
    if visible(&Band { x, y, w, h: rail_disk::height() }, vh) {
        rail_disk::draw(fb, x, y, w, &rail.sample.disk, t);
    }
}
