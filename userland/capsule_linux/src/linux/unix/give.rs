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


//! Handing a descriptor back to the client in control data.

use alloc::vec::Vec;

use crate::linux::guest::Guest;

/// struct cmsghdr: len, level, type, then the descriptors.
const CMSG_DATA: usize = 16;
const SOL_SOCKET: u32 = 1;
const SCM_RIGHTS: u32 = 1;

/// The control block for whatever is owed, or nothing when nothing is.
pub fn control(guest: &mut Guest) -> Option<Vec<u8>> {
    if guest.display.give.is_empty() {
        return None;
    }
    let fds = core::mem::take(&mut guest.display.give);
    let len = CMSG_DATA + fds.len() * 4;
    let mut out = Vec::with_capacity((len + 7) & !7);
    out.extend_from_slice(&(len as u64).to_le_bytes());
    out.extend_from_slice(&SOL_SOCKET.to_le_bytes());
    out.extend_from_slice(&SCM_RIGHTS.to_le_bytes());
    for fd in fds {
        out.extend_from_slice(&fd.to_le_bytes());
    }
    while out.len() % 8 != 0 {
        out.push(0);
    }
    Some(out)
}

/// Write it into the guest's msghdr and say how long it was, since a
/// client reads msg_controllen and not the block's own length.
pub fn place(guest: &Guest, hdr: u64, block: &[u8]) -> bool {
    let Some(raw) = guest.read(hdr, 56) else {
        return false;
    };
    let at = u64::from_le_bytes([
        raw[32], raw[33], raw[34], raw[35], raw[36], raw[37], raw[38], raw[39],
    ]);
    let room = u64::from_le_bytes([
        raw[40], raw[41], raw[42], raw[43], raw[44], raw[45], raw[46], raw[47],
    ]);
    if at == 0 || room < block.len() as u64 {
        return false;
    }
    if guest.write(at, block) < block.len() as i64 {
        return false;
    }
    guest.write(hdr + 40, &(block.len() as u64).to_le_bytes()) >= 8
}
