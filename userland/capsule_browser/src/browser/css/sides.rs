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

use super::parse_px::parse_px;

// Expand a 1-4 value box shorthand into [top, right, bottom, left].
// "auto" resolves to 0; any unparsable component rejects the whole value.
pub(super) fn sides(value: &str, em: u32, max: u32) -> Option<[u32; 4]> {
    let mut vals = [0u32; 4];
    let mut n = 0;
    for part in value.split_whitespace() {
        if n == 4 {
            return None;
        }
        vals[n] = if part.eq_ignore_ascii_case("auto") { 0 } else { parse_px(part, em)?.min(max) };
        n += 1;
    }
    match n {
        1 => Some([vals[0]; 4]),
        2 => Some([vals[0], vals[1], vals[0], vals[1]]),
        3 => Some([vals[0], vals[1], vals[2], vals[1]]),
        4 => Some(vals),
        _ => None,
    }
}
