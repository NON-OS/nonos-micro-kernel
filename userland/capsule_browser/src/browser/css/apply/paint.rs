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
use crate::browser::css::computed::{BgSize, Computed};
use crate::browser::css::parse_px::parse_px;

// Painted appearance: background color, corner radius, stacking level and
// overflow clipping.
pub(super) fn apply_paint(c: &mut Computed, name: &str, value: &str, fs: u32) -> bool {
    match name {
        "background" | "background-color" => {
            // The shorthand may carry url()/repeat tokens; take the color,
            // and a size written after the position slash.
            for part in value.split_whitespace() {
                if let Some(rgb) = parse_color(part) {
                    c.bg = rgb;
                    break;
                }
            }
            if name == "background" {
                if let Some(after) = value.split('/').nth(1) {
                    apply_bg_size(c, after, fs);
                }
                if value.contains("no-repeat") {
                    c.bg_repeat = false;
                }
            }
        }
        "background-size" => apply_bg_size(c, value, fs),
        "background-repeat" => {
            c.bg_repeat = !value.split(',').next().unwrap_or("").contains("no-repeat");
        }
        // One radius for all corners; the first value of a corner list wins.
        "border-radius" => {
            if let Some(first) = value.split_whitespace().next() {
                if let Some(px) = parse_px(first, fs) {
                    c.radius = px.min(64);
                }
            }
        }
        "overflow" | "overflow-x" | "overflow-y" => match value.trim() {
            "hidden" | "clip" => c.overflow_hidden = true,
            "visible" | "auto" | "scroll" => c.overflow_hidden = false,
            _ => {}
        },
        "opacity" => {
            if let Ok(v) = value.trim().parse::<f32>() {
                if (0.0..=1.0).contains(&v) {
                    c.opacity = (v * 255.0) as u8;
                }
            }
        }
        // visibility keeps the box's space but paints nothing, which zero
        // opacity models; descendant re-show is rare enough to ignore.
        "visibility" => match value.trim() {
            "hidden" | "collapse" => c.opacity = 0,
            "visible" => c.opacity = 255,
            _ => {}
        },
        "z-index" => {
            if let Ok(z) = value.trim().parse::<i32>() {
                c.z = z.clamp(-999, 999);
            }
        }
        _ => return false,
    }
    true
}

// The first layer of a background-size list: cover, contain, auto, or a
// length that scales the tile width with the aspect kept.
fn apply_bg_size(c: &mut Computed, value: &str, fs: u32) {
    let first = value.split(',').next().unwrap_or("").trim();
    let head = first.split_whitespace().next().unwrap_or("");
    c.bg_size = match head {
        "cover" => BgSize::Cover,
        "contain" => BgSize::Contain,
        "auto" | "" => BgSize::Auto,
        len => match parse_px(len, fs) {
            Some(px) if px > 0 => BgSize::Px(px.min(u16::MAX as u32) as u16),
            _ => return,
        },
    };
}
