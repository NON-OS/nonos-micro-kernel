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

use alloc::vec::Vec;

use super::split::split_top;

// A parsed linear gradient: an angle in CSS degrees (0 points up, clockwise)
// and color stops with positions in 0..1.
pub(super) struct Linear {
    pub angle: f32,
    pub stops: Vec<(u32, f32)>,
}

// Parse linear-gradient(...) content. The first item may be an angle or a
// "to <side>" direction; the rest are color stops. Positions left blank are
// spread evenly. Returns None for a non-linear or unparseable value.
pub(super) fn parse_linear(func: &str) -> Option<Linear> {
    let inner = func.strip_prefix("linear-gradient(")?.strip_suffix(')')?;
    let mut items = split_top(inner);
    if items.is_empty() {
        return None;
    }
    let mut angle = 180.0;
    if let Some(a) = direction(items[0].trim()) {
        angle = a;
        items.remove(0);
    }
    let stops = super::stop_list::parse_stops(&items)?;
    Some(Linear { angle, stops })
}

// A leading angle or side keyword; None means the item is a color stop.
fn direction(s: &str) -> Option<f32> {
    if let Some(deg) = s.strip_suffix("deg") {
        return deg.trim().parse::<f32>().ok();
    }
    let d = s.strip_prefix("to ")?.trim();
    Some(match d {
        "top" => 0.0,
        "right" => 90.0,
        "bottom" => 180.0,
        "left" => 270.0,
        "top right" | "right top" => 45.0,
        "bottom right" | "right bottom" => 135.0,
        "bottom left" | "left bottom" => 225.0,
        _ => 315.0,
    })
}
