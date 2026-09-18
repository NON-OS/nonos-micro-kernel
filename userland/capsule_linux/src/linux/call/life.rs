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

//! Ending a guest. The call never returns to the guest, so the answer
//! handed back is only what parks it until the supervisor tears it down.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

pub fn exit_thread(guest: &mut Guest, tid: u32) -> u64 {
    guest.threads.retain(|t| *t != tid);
    errno::ok(0)
}

pub fn exit(guest: &mut Guest, code: u64) -> u64 {
    guest.exited = Some(code as i32);
    errno::ok(0)
}
