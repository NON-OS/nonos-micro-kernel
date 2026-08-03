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
//! Framing one packet.

extern crate alloc;

use alloc::vec::Vec;

use super::hex;

/// Frame `payload` as a pkt-line, length header included.
pub fn encode_pkt(payload: &[u8], out: &mut Vec<u8>) {
    let len = payload.len() + 4;
    for shift in [12, 8, 4, 0] {
        out.push(hex::digit(((len >> shift) & 0xF) as u8));
    }
    out.extend_from_slice(payload);
}
