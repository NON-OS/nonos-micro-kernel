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

//! The controller driver's reply, caught instead of sent.

use std::cell::RefCell;

use crate::raw;

thread_local! {
    static REPLY: RefCell<Option<(u32, Vec<u8>)>> = const { RefCell::new(None) };
}

/// `mk_ipc_reply(dest_pid, buf, len)`: the bytes the handler would have sent
/// to `dest_pid`, kept for the caller of the handler to collect.
pub fn mk_ipc_reply(dest_pid: u32, buf: *const u8, len: usize) -> i64 {
    let bytes = raw::bytes(buf, len).to_vec();
    REPLY.with(|r| *r.borrow_mut() = Some((dest_pid, bytes)));
    len as i64
}

/// The last reply and who it was for, leaving the mailbox empty.
pub fn take_reply() -> Option<(u32, Vec<u8>)> {
    REPLY.with(|r| r.borrow_mut().take())
}
