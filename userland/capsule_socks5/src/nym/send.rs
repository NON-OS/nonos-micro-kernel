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

use super::exit::exit;
use super::session::session;
use crate::conn::Dest;
use crate::tunnel::encode_connect;
use alloc::vec;
use alloc::vec::Vec;

/// Largest request this capsule will build. The mixnet payload is fixed
/// width, so anything past it has to be split rather than truncated.
const REQUEST_MAX: usize = 2048;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SendError {
    NoExit,
    NoSession,
    TooLarge,
}

/// Build the network-requester frame that asks an exit to open `dest`.
///
/// Returns the bytes to hand to `net.nym`. Refuses when no exit is configured:
/// there is no safe default, and falling back to a direct socket would defeat
/// the reason this capsule exists.
pub fn connect_request(conn_id: u64, dest: &Dest) -> Result<Vec<u8>, SendError> {
    if exit().is_none() {
        return Err(SendError::NoExit);
    }
    if session().is_none() {
        return Err(SendError::NoSession);
    }
    let mut buf = vec![0u8; REQUEST_MAX];
    let n = encode_connect(conn_id, dest, &mut buf).ok_or(SendError::TooLarge)?;
    buf.truncate(n);
    Ok(buf)
}
