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

// A paint value to opaque ARGB. None means "do not paint" (none/transparent)
// or a server we cannot resolve (url() gradients). currentColor maps to a
// light neutral: the embedding text color is not known at decode time and
// icons land on the dark theme far more often than not.
pub(super) fn parse_paint(v: &str) -> Option<u32> {
    let t = v.trim();
    let low = t.to_ascii_lowercase();
    match low.as_str() {
        "none" | "transparent" => return None,
        "currentcolor" => return Some(0xFFC8_D2DC),
        "black" => return Some(0xFF00_0000),
        "white" => return Some(0xFFFF_FFFF),
        "red" => return Some(0xFFFF_0000),
        "green" => return Some(0xFF00_8000),
        "blue" => return Some(0xFF00_00FF),
        "gray" | "grey" => return Some(0xFF80_8080),
        _ => {}
    }
    if low.starts_with("url(") {
        return None;
    }
    if let Some(hex) = low.strip_prefix('#') {
        let d = |c: u8| (c as char).to_digit(16);
        let b = hex.as_bytes();
        if hex.len() == 3 {
            let (r, g, bl) = (d(b[0])?, d(b[1])?, d(b[2])?);
            return Some(0xFF00_0000 | (r * 17) << 16 | (g * 17) << 8 | (bl * 17));
        }
        if hex.len() >= 6 {
            let mut v = 0u32;
            for &c in &b[..6] {
                v = (v << 4) | d(c)?;
            }
            return Some(0xFF00_0000 | v);
        }
        return None;
    }
    if let Some(args) = low.strip_prefix("rgb(").and_then(|r| r.strip_suffix(')')) {
        let n = super::num::num_list(args);
        if n.len() >= 3 {
            let ch = |x: f32| (x.clamp(0.0, 255.0)) as u32;
            return Some(0xFF00_0000 | ch(n[0]) << 16 | ch(n[1]) << 8 | ch(n[2]));
        }
    }
    None
}
