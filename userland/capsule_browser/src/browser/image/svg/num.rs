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

// Whitespace/comma separated float list, as used by viewBox, points and
// transform arguments.
pub(super) fn num_list(s: &str) -> Vec<f32> {
    s.split(|c: char| c.is_whitespace() || c == ',')
        .filter(|t| !t.is_empty())
        .filter_map(|t| t.parse::<f32>().ok())
        .collect()
}

// A length attribute in user units; the px suffix is the only unit icons
// carry in practice. Percentages and other units yield None.
pub(super) fn parse_len(s: &str) -> Option<f32> {
    let t = s.trim();
    let t = t.strip_suffix("px").unwrap_or(t);
    let v = t.trim().parse::<f32>().ok()?;
    if v.is_finite() {
        Some(v)
    } else {
        None
    }
}
