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

// A gradient stop color, keeping the alpha the CSS color parser drops, since
// semi-transparent stops are what make an overlay gradient work. Handles hex
// with an alpha byte and rgb/rgba functions; falls back to the opaque parse.
pub(super) fn stop_color(v: &str) -> Option<u32> {
    let t = v.trim();
    if let Some(hex) = t.strip_prefix('#') {
        return hex_alpha(hex);
    }
    if let Some(args) = t.strip_prefix("rgba(").or_else(|| t.strip_prefix("rgb(")) {
        let n: alloc::vec::Vec<f32> = args
            .trim_end_matches(')')
            .split(',')
            .filter_map(|p| p.trim().trim_end_matches('%').parse::<f32>().ok())
            .collect();
        if n.len() >= 3 {
            let a = if n.len() >= 4 { (n[3] * 255.0) as u32 } else { 255 };
            let ch = |x: f32| x.clamp(0.0, 255.0) as u32;
            return Some((a << 24) | (ch(n[0]) << 16) | (ch(n[1]) << 8) | ch(n[2]));
        }
    }
    crate::browser::css::parse_color(t)
}

fn hex_alpha(h: &str) -> Option<u32> {
    let d = |c: u8| (c as char).to_digit(16);
    let b = h.as_bytes();
    let (r, g, bl, a) = match h.len() {
        3 => (d(b[0])? * 17, d(b[1])? * 17, d(b[2])? * 17, 255),
        4 => (d(b[0])? * 17, d(b[1])? * 17, d(b[2])? * 17, d(b[3])? * 17),
        6 => (hx(b, 0)?, hx(b, 2)?, hx(b, 4)?, 255),
        8 => (hx(b, 0)?, hx(b, 2)?, hx(b, 4)?, hx(b, 6)?),
        _ => return None,
    };
    Some((a << 24) | (r << 16) | (g << 8) | bl)
}

fn hx(b: &[u8], i: usize) -> Option<u32> {
    let hi = (b[i] as char).to_digit(16)?;
    let lo = (b[i + 1] as char).to_digit(16)?;
    Some(hi * 16 + lo)
}
