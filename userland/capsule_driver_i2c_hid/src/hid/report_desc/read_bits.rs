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

//! Read a little-endian bit field from a report body. HID packs fields LSB
//! first within each byte. Any bit that falls past the end of the buffer reads
//! as zero, so a short report can never cause an out-of-range access.

pub(super) fn read_bits(body: &[u8], bit_offset: u32, bit_size: u32) -> u32 {
    let mut value = 0u32;
    for k in 0..bit_size.min(32) {
        let bit = bit_offset + k;
        let byte = (bit / 8) as usize;
        if byte >= body.len() {
            break;
        }
        let set = (body[byte] >> (bit % 8)) & 1;
        value |= (set as u32) << k;
    }
    value
}
