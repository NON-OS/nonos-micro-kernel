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

/// Penalty for finder-like 1:1:3:1:1 runs bordered by four light modules,
/// scanned along one row or column; 40 points each.
pub(super) fn finder_penalty(n: usize, get: impl Fn(usize) -> bool) -> u32 {
    let mut score = 0u32;
    for i in 0..n {
        if i + 11 > n {
            break;
        }
        let w: [bool; 11] = core::array::from_fn(|k| get(i + k));
        let core = [true, false, true, true, true, false, true];
        let left = w[0..7] == core && w[7..11] == [false; 4];
        let right = w[4..11] == core && w[0..4] == [false; 4];
        if left || right {
            score += 40;
        }
    }
    score
}
