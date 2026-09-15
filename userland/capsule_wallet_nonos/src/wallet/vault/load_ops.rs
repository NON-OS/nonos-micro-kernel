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

//! Opening the wallet file. The read is in [`super::load_read`].

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::mk_getpid;

use super::answer::Answer;
use super::vfs::{call, HDR_LEN, OP_OPEN};

/// The descriptor, or which kind of nothing.
///
/// `Silent` is not `absent`: the store is deaf while it stages packages at
/// boot, and a window that opened in that window would otherwise conclude the
/// machine has no wallet and never ask again.
pub(super) enum Opened {
    Fd(u32),
    Absent,
    Silent,
}

/// No `O_CREATE`: a wallet that has never been saved must read as absent, not
/// be brought into existence as an empty file.
pub(super) fn open_existing(path: &[u8]) -> Opened {
    let pid = mk_getpid();
    let mut body = Vec::with_capacity(9 + path.len());
    body.extend_from_slice(&pid.to_le_bytes());
    body.push(path.len() as u8);
    body.extend_from_slice(path);
    body.extend_from_slice(&0u32.to_le_bytes());
    let mut rx = vec![0u8; 64];
    let total = match call(OP_OPEN, &body, &mut rx) {
        Answer::Ok(n) => n,
        Answer::Refused => return Opened::Absent,
        Answer::Silent => return Opened::Silent,
    };
    if total < HDR_LEN + 8 {
        return Opened::Absent;
    }
    Opened::Fd(u32::from_le_bytes([
        rx[HDR_LEN + 4],
        rx[HDR_LEN + 5],
        rx[HDR_LEN + 6],
        rx[HDR_LEN + 7],
    ]))
}
