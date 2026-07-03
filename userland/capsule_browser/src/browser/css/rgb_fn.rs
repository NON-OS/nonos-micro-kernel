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

// Parse rgb()/rgba(): the first three components are the channels, honoring a
// trailing % form; any alpha is ignored. Returns an opaque ARGB value.
pub(super) fn parse_rgb(s: &str) -> Option<u32> {
    let open = s.find('(')?;
    let inner = s.get(open + 1..)?;
    let inner = inner.strip_suffix(')').unwrap_or(inner);
    let mut chan = [0u32; 3];
    let mut n = 0;
    for tok in inner.split(|c: char| c == ',' || c == '/' || c.is_ascii_whitespace()) {
        let tok = tok.trim();
        if tok.is_empty() {
            continue;
        }
        if n == 3 {
            break;
        }
        chan[n] = channel(tok)?;
        n += 1;
    }
    if n < 3 {
        return None;
    }
    Some(0xFF00_0000 | (chan[0] << 16) | (chan[1] << 8) | chan[2])
}

// One 0-255 channel, accepting either a number or an NN% form.
fn channel(tok: &str) -> Option<u32> {
    if let Some(pct) = tok.strip_suffix('%') {
        let f = pct.trim().parse::<f32>().ok()?;
        if !f.is_finite() {
            return None;
        }
        return Some((f.clamp(0.0, 100.0) * 255.0 / 100.0 + 0.5) as u32);
    }
    let f = tok.parse::<f32>().ok()?;
    if !f.is_finite() {
        return None;
    }
    Some((f.clamp(0.0, 255.0) + 0.5) as u32)
}
