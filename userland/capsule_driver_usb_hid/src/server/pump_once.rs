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

use crate::protocol::parse;
use crate::server::dispatch::dispatch;
use crate::state::State;

const SERVICE_INBOX: u64 = 0;
const RECV_TIMEOUT_MS: u64 = 1;

pub fn pump_once(state: &mut State, rx: &mut [u8], tx: &mut [u8]) -> bool {
    let mut sender_pid = 0u32;
    let n = mk_ipc_recv_from(
        SERVICE_INBOX,
        rx.as_mut_ptr(),
        rx.len(),
        RECV_TIMEOUT_MS,
        &mut sender_pid,
    );
    if n <= 0 || sender_pid == 0 {
        return false;
    }
    let Some((req, body)) = parse(&rx[..n as usize]) else {
        return false;
    };
    dispatch(state, sender_pid, req, body, tx);
    true
}
