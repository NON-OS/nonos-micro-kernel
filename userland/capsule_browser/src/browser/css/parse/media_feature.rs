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

use crate::browser::manifest::HEIGHT;

// Verdict for one parenthesized media feature against the fixed desktop
// viewport: mouse pointer, hover, landscape, light scheme, 1x resolution.
// Unknown features fail closed so a block written for capabilities we lack
// cannot restyle the page.
pub(super) fn feature_matches(body: &str, viewport_w: u32) -> bool {
    let (name, value) = match body.find(':') {
        Some(c) => (body[..c].trim(), Some(body[c + 1..].trim())),
        None => (body.trim(), None),
    };
    match (name, value) {
        ("max-width", Some(v)) => len_px(v).is_some_and(|px| viewport_w <= px),
        ("min-width", Some(v)) => len_px(v).is_some_and(|px| viewport_w >= px),
        ("width", Some(v)) => len_px(v) == Some(viewport_w),
        ("max-height", Some(v)) => len_px(v).is_some_and(|px| HEIGHT <= px),
        ("min-height", Some(v)) => len_px(v).is_some_and(|px| HEIGHT >= px),
        ("orientation", Some(v)) => v == "landscape",
        ("prefers-color-scheme", Some(v)) => v == "light",
        ("prefers-reduced-motion" | "prefers-contrast", Some(v)) => v == "no-preference",
        ("hover" | "any-hover", Some(v)) => v == "hover",
        ("pointer" | "any-pointer", Some(v)) => v == "fine",
        ("hover" | "any-hover" | "pointer" | "any-pointer", None) => true,
        // 1x device: any maximum accommodates it, any explicit minimum is
        // treated as asking for more.
        ("max-resolution", _) => true,
        _ => false,
    }
}

// px, em and rem bounds; media queries resolve em against the 16px initial
// font size, never the element's.
fn len_px(v: &str) -> Option<u32> {
    let v = v.trim();
    if let Some(n) = v.strip_suffix("px") {
        return n.trim().parse::<f32>().ok().map(|f| (f + 0.5) as u32);
    }
    for unit in ["rem", "em"] {
        if let Some(n) = v.strip_suffix(unit) {
            return n.trim().parse::<f32>().ok().map(|f| (f * 16.0 + 0.5) as u32);
        }
    }
    None
}
