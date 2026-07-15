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

/// REQUEST SENSE (SPC opcode 0x03): fetch the sense data that explains the last
/// CHECK CONDITION. `alloc_len` bounds how many sense bytes the device returns
/// and lands in the allocation-length field. Returns the 16-byte CDB and its
/// significant length.
pub fn request_sense(alloc_len: u8) -> ([u8; 16], u8) {
    let mut cdb = [0u8; 16];
    cdb[0] = 0x03;
    cdb[4] = alloc_len;
    (cdb, 6)
}
