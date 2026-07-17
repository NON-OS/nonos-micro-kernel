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

pub(super) const MAX_ARRAY: usize = 100_000;

// Resolve a possibly-negative slice index against a length, clamped to [0, len].
pub(super) fn clamp_index(v: f64, len: usize) -> usize {
    let len = len as i64;
    let i = v as i64;
    let i = if i < 0 { len + i } else { i };
    i.clamp(0, len) as usize
}
