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

use super::condition::condition;
use crate::matrix::Matrix;

/// Flip every non-function module where the mask condition holds.
pub(crate) fn apply(m: &mut Matrix, mask: u8) {
    let n = m.n;
    for y in 0..n {
        for x in 0..n {
            if !m.function[y * n + x] && condition(mask, y, x) {
                m.modules[y * n + x] ^= true;
            }
        }
    }
}
