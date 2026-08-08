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
//! Closing a socket.

use super::super::call::call;
use super::super::constants::OP_CLOSE;

/// Release the handle. A close that fails leaves the socket to be reaped when
/// the capsule exits, so there is nothing useful for a caller to do about it.
pub fn close(port: u32, handle: u32) -> bool {
    let mut body = [0u8; 4];
    let mut rx = [0u8; 20];
    body.copy_from_slice(&handle.to_le_bytes());
    call(port, OP_CLOSE, &body, &mut rx).is_ok()
}
