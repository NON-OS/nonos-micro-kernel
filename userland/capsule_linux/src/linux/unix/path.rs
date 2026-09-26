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


//! `struct sockaddr_un` out of a guest, and which server it names.

use alloc::vec::Vec;

use crate::linux::guest::Guest;

const AF_UNIX: u16 = 1;
/// family(2) then a 108 byte path.
const SUN_LEN: usize = 110;

/// The display socket a Wayland client looks for.
const DISPLAY: &[u8] = b"wayland-0";

pub fn sun_path(guest: &Guest, at: u64, len: u64) -> Option<Vec<u8>> {
    let take = (len as usize).min(SUN_LEN);
    if take < 3 {
        return None;
    }
    let raw = guest.read(at, take)?;
    if u16::from_le_bytes([raw[0], raw[1]]) != AF_UNIX {
        return None;
    }
    let body = &raw[2..];
    let end = body.iter().position(|b| *b == 0).unwrap_or(body.len());
    Some(body[..end].to_vec())
}

/// True when the path ends in the display socket's name, whatever
/// directory the client was told to look in.
pub fn is_display(path: &[u8]) -> bool {
    path.ends_with(DISPLAY)
}
