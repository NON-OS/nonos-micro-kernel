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

//! A query in, an answer out, and nothing on the wire.

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Kind};

use super::decide::answer;

/// Take a query and queue its answer, to appear to come back from `peer`,
/// which is whatever the program believes its nameserver is.
pub fn query(guest: &mut Guest, fd: u64, buf: u64, len: u64, peer: [u8; 6]) -> u64 {
    let Some(msg) = guest.read(buf, len as usize) else {
        return errno::fail(errno::EFAULT);
    };
    if let Some(answer) = answer(guest, &msg) {
        if let Some(entry) = guest.fds.get_mut(fd as usize).filter(|f| f.kind == Kind::Resolver) {
            entry.replies.push((answer, peer));
        }
    }
    errno::ok(len)
}

/// The oldest answer waiting, with the address it came from, or nothing.
pub fn answer_out(guest: &mut Guest, fd: u64, buf: u64, len: u64) -> (u64, Option<[u8; 6]>) {
    let Some(entry) = guest.fds.get_mut(fd as usize).filter(|f| f.kind == Kind::Resolver) else {
        return (errno::fail(errno::EBADF), None);
    };
    if entry.replies.is_empty() {
        return (errno::fail(errno::EAGAIN), None);
    }
    let (msg, peer) = entry.replies.remove(0);
    let take = msg.len().min(len as usize);
    match guest.write(buf, &msg[..take]) {
        n if n < take as i64 => (errno::fail(errno::EFAULT), None),
        _ => (errno::ok(take as u64), Some(peer)),
    }
}
