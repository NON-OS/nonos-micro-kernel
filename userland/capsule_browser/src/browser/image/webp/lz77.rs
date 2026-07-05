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

use super::bitread::BitReader;

// A length or distance prefix code expands to a base value plus extra bits.
// Codes 0 and 1 are literal; from 2 up, each pair of codes doubles the range,
// so a small symbol space covers large runs and offsets.
pub(super) fn prefix_value(code: u32, br: &mut BitReader) -> u32 {
    if code < 4 {
        return code + 1;
    }
    let extra = (code - 2) >> 1;
    let offset = (2 + (code & 1)) << extra;
    offset + br.read(extra) + 1
}
