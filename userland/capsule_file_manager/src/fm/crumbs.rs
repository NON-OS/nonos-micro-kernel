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

extern crate alloc;

use alloc::{string::String, vec::Vec};

use nonos_app_skeleton::measure_ttf;

use super::header_layout::strip_left;
use super::header_nav::nav_right;
use super::header_slots::{
    HeadHit, Slot, CRUMB_PAD, CRUMB_PX, CRUMB_ROOT, CRUMB_SEP_W, HOME_W, TOOL_GAP,
};
use super::state::State;

/// The current prefix as breadcrumb segments, root first. Index 0 is the root,
/// so `HeadHit::Crumb(i)` indexes straight into this.
pub fn crumb_parts(state: &State) -> Vec<String> {
    let mut parts = Vec::new();
    parts.push(String::from(CRUMB_ROOT));
    for segment in state.prefix.split('/') {
        if !segment.is_empty() {
            parts.push(String::from(segment));
        }
    }
    parts
}

/// Left edge of the first breadcrumb segment, inset from the plate that wraps
/// the trail. The plate itself is derived back from this in the painter.
pub fn crumb_x() -> u32 {
    nav_right() + TOOL_GAP + CRUMB_PAD
}

/// Where each segment is drawn. The painter walks these and `head_hit` tests
/// against them, so a crumb click always lands on the segment under the cursor.
/// A segment that would run under the control strip is dropped from both. The
/// root is a home glyph of fixed width rather than measured text, and each
/// separator is a chevron drawn into the gap reserved ahead of its segment.
pub fn crumb_slots(state: &State) -> Vec<Slot> {
    let limit = strip_left(state).saturating_sub(CRUMB_PAD);
    let mut out = Vec::new();
    let mut x = crumb_x();
    for (i, part) in crumb_parts(state).iter().enumerate() {
        if i > 0 {
            x += CRUMB_SEP_W;
        }
        let w = if i == 0 { HOME_W } else { measure_ttf(part, CRUMB_PX).max(0) as u32 };
        if x + w > limit {
            break;
        }
        out.push(Slot { x, w, hit: HeadHit::Crumb(i) });
        x += w;
    }
    out
}
