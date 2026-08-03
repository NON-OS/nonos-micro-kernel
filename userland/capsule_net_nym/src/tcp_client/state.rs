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

const OP_STATE: u16 = 9;

pub const SYN_SENT: u8 = 1;
pub const SYN_RECEIVED: u8 = 2;
pub const ESTABLISHED: u8 = 3;
pub const CLOSED: u8 = 0xFF;

/// One-byte connection state from `net.tcp`, mapped from the smoltcp state
/// machine. Read-only: it neither drains the socket nor advances it.
pub fn state(port: u32, handle: u32) -> Result<u8, u16> {
    let mut out = [0u8; 1];
    if call(port, OP_STATE, &handle.to_le_bytes(), &mut out)? != 1 {
        return Err(4);
    }
    Ok(out[0])
}
