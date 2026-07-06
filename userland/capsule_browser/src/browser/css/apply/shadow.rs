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

use crate::browser::css::color::parse_color;
use crate::browser::css::computed::{Computed, Shadow};
use crate::browser::css::parse_px::parse_px;

// box-shadow: offset-x offset-y [blur] [spread] [color]. Inset shadows and
// the second (inner) layer are ignored; the outer drop shadow is what carries
// the visual weight. `none` clears it.
pub(super) fn apply_shadow(c: &mut Computed, name: &str, value: &str, fs: u32) -> bool {
    if name != "box-shadow" {
        return false;
    }
    let v = value.trim();
    if v == "none" || v.starts_with("inset") {
        c.shadow = if v == "none" { None } else { c.shadow };
        return true;
    }
    // Only the first shadow layer before a comma is drawn.
    let layer = v.split(',').next().unwrap_or(v);
    let mut nums = [0i32; 2];
    let mut got = 0usize;
    let mut color = 0u32;
    let mut blur = 0u32;
    let mut idx = 0usize;
    for tok in layer.split_whitespace() {
        if let Some(px) = parse_px(tok, fs).map(|p| p as i32).or_else(|| signed_px(tok, fs)) {
            match idx {
                0 | 1 => {
                    nums[idx] = px;
                    got += 1;
                }
                2 => blur = px.max(0) as u32,
                _ => {}
            }
            idx += 1;
        } else if let Some(rgb) = parse_color(tok) {
            color = rgb;
        }
    }
    if got >= 2 {
        let color = if color != 0 { color } else { 0x6600_0000 };
        c.shadow = Some(Shadow { dx: nums[0], dy: nums[1], blur, color });
    }
    true
}

// A length that may be negative, which the unsigned px parser rejects.
fn signed_px(tok: &str, fs: u32) -> Option<i32> {
    let neg = tok.starts_with('-');
    let mag = parse_px(tok.trim_start_matches('-'), fs)? as i32;
    Some(if neg { -mag } else { mag })
}
