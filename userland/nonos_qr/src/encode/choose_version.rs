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

use crate::version::{blocks, Ecc};

/// The smallest version 1..=10 whose byte-mode capacity holds `len` data bytes
/// at this EC level, or None when it does not fit.
pub(crate) fn choose_version(len: usize, ecc: Ecc) -> Option<u8> {
    for v in 1..=10u8 {
        let cap = blocks(v, ecc).total_data_codewords();
        let count_bits = if v <= 9 { 8 } else { 16 };
        let needed = (4 + count_bits + len * 8).div_ceil(8);
        if needed <= cap {
            return Some(v);
        }
    }
    None
}
