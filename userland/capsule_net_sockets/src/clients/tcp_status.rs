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

use super::envelope::call;

const MAGIC: u32 = 0x4E54_4350;
const STATE: u16 = 9;
const POLL: u16 = 10;
const E_LEN: u16 = 4;

/// TCP state code for a connection: 3 = established, 0xFF = closed.
pub fn state(port: u32, handle: u32) -> Result<u8, u16> {
    one_byte(port, STATE, handle)
}

/// Non-consuming readiness bits (bit0 readable, bit1 writable). The async
/// reactor polls this across its sockets to decide which tasks to wake, so it
/// must never drain the connection.
pub fn poll(port: u32, handle: u32) -> Result<u8, u16> {
    one_byte(port, POLL, handle)
}

fn one_byte(port: u32, op: u16, handle: u32) -> Result<u8, u16> {
    let mut out = [0u8; 1];
    if call(port, MAGIC, op, &handle.to_le_bytes(), &mut out)? != 1 {
        return Err(E_LEN);
    }
    Ok(out[0])
}
