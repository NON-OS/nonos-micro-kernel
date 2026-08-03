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

use super::types::BLOCK_BYTES;

/// Row r shifts left by r. State is column-major, so row r of column c lives
/// at c * 4 + r.
pub(crate) fn shift_rows(block: &mut [u8; BLOCK_BYTES]) {
    let src = *block;
    for row in 1..4 {
        for col in 0..4 {
            block[col * 4 + row] = src[((col + row) % 4) * 4 + row];
        }
    }
}
