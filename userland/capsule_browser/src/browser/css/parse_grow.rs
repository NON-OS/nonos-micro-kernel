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

const MAX_GROW: u32 = 100;

// flex-grow factor rounded to whole units, capped to keep the math sane.
pub(super) fn parse_grow(value: &str) -> Option<u32> {
    let f = value.trim().parse::<f32>().ok()?;
    if f.is_finite() && (0.0..=MAX_GROW as f32).contains(&f) {
        Some((f + 0.5) as u32)
    } else {
        None
    }
}
