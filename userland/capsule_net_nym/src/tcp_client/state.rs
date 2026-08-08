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

/// Added to whatever the call reported, so the underlying code survives
/// instead of every failure arriving as one value.
pub const E_STATE_CALL: u16 = 200;
/// net.tcp answered with something other than the one state byte.
pub const E_STATE_LEN: u16 = 9;

/// One-byte connection state from `net.tcp`, mapped from the smoltcp state
/// machine. Read-only: it neither drains the socket nor advances it.
///
/// The two failure modes are kept apart because they mean different things:
/// a failed call is the service refusing or unreachable, a wrong length is
/// the two sides disagreeing about the reply.
pub fn state(port: u32, handle: u32) -> Result<u8, u16> {
    let mut out = [0u8; 1];
    match call(port, OP_STATE, &handle.to_le_bytes(), &mut out) {
        Ok(1) => Ok(out[0]),
        Ok(_) => Err(E_STATE_LEN),
        Err(e) => Err(E_STATE_CALL + e),
    }
}
