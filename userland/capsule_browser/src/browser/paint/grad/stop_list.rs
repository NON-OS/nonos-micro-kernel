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

use super::color::stop_color;

// Parse a comma-split list of "color [position%]" items into stops with
// positions in 0..1. Blank positions spread evenly between the ends. Returns
// None unless at least two stops parse, so a degenerate gradient is skipped.
pub(super) fn parse_stops(items: &[alloc::string::String]) -> Option<Vec<(u32, f32)>> {
    let mut stops: Vec<(u32, f32)> = Vec::new();
    let n = items.len();
    for (i, item) in items.iter().enumerate() {
        let mut parts = item.trim().rsplitn(2, ' ');
        let last = parts.next().unwrap_or("");
        let (color_str, pos) = match last.strip_suffix('%') {
            Some(p) => (parts.next().unwrap_or("").trim(), p.trim().parse::<f32>().ok()),
            None => (item.trim(), None),
        };
        let color = stop_color(color_str)?;
        let even = if n > 1 { i as f32 / (n - 1) as f32 } else { 0.0 };
        stops.push((color, pos.map(|v| v / 100.0).unwrap_or(even).clamp(0.0, 1.0)));
    }
    (stops.len() >= 2).then_some(stops)
}
