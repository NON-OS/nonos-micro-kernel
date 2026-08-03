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

use super::exit::Exit;
use crate::ipc::{call, OP_SET_DESTINATION};
use alloc::vec::Vec;

/// Tell the mixnet capsule where a session's traffic is bound.
///
/// The identifier is the reply-block tag the exit echoes back. Taking it from
/// the encryption key's leading half distinguishes one session from another
/// without carrying anything the exit could link us by.
pub fn bind_destination(id: u32, exit: &Exit) -> Result<(), ()> {
    let mut body: Vec<u8> = Vec::with_capacity(4 + 32 + 16);
    body.extend_from_slice(&id.to_le_bytes());
    body.extend_from_slice(&exit.identity);
    body.extend_from_slice(&exit.encryption[..16]);
    call(OP_SET_DESTINATION, &body).map(|_| ()).map_err(|_| ())
}
