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

use nonos_libc::mk_ipc_recv_from;

const SERVICE_INBOX: u64 = 0;
const POLL_MS: u64 = 50;

pub fn receive(rx: &mut [u8], sender_pid: &mut u32) -> i64 {
    mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), POLL_MS, sender_pid)
}
