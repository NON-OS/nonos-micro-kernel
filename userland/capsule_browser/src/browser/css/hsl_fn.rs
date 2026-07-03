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

// Parse hsl()/hsla(): hue in degrees, saturation and lightness in percent.
// Alpha is ignored; the result is an opaque ARGB value.
pub(super) fn parse_hsl(s: &str) -> Option<u32> {
    let open = s.find('(')?;
    let inner = s.get(open + 1..)?;
    let inner = inner.strip_suffix(')').unwrap_or(inner);
    let mut vals = [0f32; 3];
    let mut n = 0;
    for tok in inner.split(|c: char| c == ',' || c == '/' || c.is_ascii_whitespace()) {
        let tok = tok.trim();
        if tok.is_empty() {
            continue;
        }
        if n == 3 {
            break;
        }
        let num = tok.trim_end_matches('%').trim_end_matches("deg");
        let f = num.trim().parse::<f32>().ok()?;
        if !f.is_finite() {
            return None;
        }
        vals[n] = f;
        n += 1;
    }
    if n < 3 {
        return None;
    }
    let (r, g, b) = hsl_to_rgb(vals[0], vals[1], vals[2]);
    Some(0xFF00_0000 | ((r as u32) << 16) | ((g as u32) << 8) | b as u32)
}

// Integer HSL to RGB in per-mille fixed point, avoiding any libm calls.
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let h = if h.is_finite() { (h + 0.5) as i32 } else { 0 };
    let h = h.rem_euclid(360);
    let s = ((if s.is_finite() { s + 0.5 } else { 0.0 }) as i32).clamp(0, 100) * 10;
    let l = ((if l.is_finite() { l + 0.5 } else { 0.0 }) as i32).clamp(0, 100) * 10;
    let two_l = 2 * l - 1000;
    let abs_l = if two_l < 0 { -two_l } else { two_l };
    let c = (1000 - abs_l) * s / 1000;
    let hs = h * 1000 / 60;
    let hmod = hs % 2000;
    let d = if hmod < 1000 { 1000 - hmod } else { hmod - 1000 };
    let x = c * (1000 - d) / 1000;
    let m = l - c / 2;
    let (r1, g1, b1) = match h / 60 {
        0 => (c, x, 0),
        1 => (x, c, 0),
        2 => (0, c, x),
        3 => (0, x, c),
        4 => (x, 0, c),
        _ => (c, 0, x),
    };
    let to8 = |v: i32| ((v + m) * 255 / 1000).clamp(0, 255) as u8;
    (to8(r1), to8(g1), to8(b1))
}
