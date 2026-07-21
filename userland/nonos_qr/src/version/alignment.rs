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

/// Alignment-pattern centre coordinates for a version (empty for v1). The full
/// centre set is the cartesian product of these, minus the three that collide
/// with the finder patterns.
pub(crate) fn alignment_positions(version: u8) -> &'static [u8] {
    const P: [&[u8]; 10] = [
        &[],
        &[6, 18],
        &[6, 22],
        &[6, 26],
        &[6, 30],
        &[6, 34],
        &[6, 22, 38],
        &[6, 24, 42],
        &[6, 26, 46],
        &[6, 28, 50],
    ];
    P[(version - 1) as usize]
}
