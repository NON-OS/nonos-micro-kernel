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


use super::consts::PCR_SELECT_BYTES;

/// PCR indices to a bitmap, low bit of each byte first, which is how the spec
/// orders selection bits. Indices past the bank are dropped rather than
/// wrapping into an unrelated PCR.
pub(super) fn pcr_bitmap(pcrs: &[u8]) -> [u8; PCR_SELECT_BYTES] {
    let mut bits = [0u8; PCR_SELECT_BYTES];
    for &pcr in pcrs {
        let byte = pcr as usize / 8;
        if byte < PCR_SELECT_BYTES {
            bits[byte] |= 1 << (pcr % 8);
        }
    }
    bits
}
