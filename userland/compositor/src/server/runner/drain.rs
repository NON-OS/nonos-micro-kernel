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

use super::dispatch::dispatch;
use crate::protocol::parse;
use crate::server::respond;
use crate::state::Context;

const SERVICE_INBOX: u64 = 0;
const RECV_NOWAIT: u64 = 1;
const MAX_BATCH: usize = 16;

pub fn drain_ipc(ctx: &mut Context, rx: &mut [u8], tx: &mut [u8]) {
    for _ in 0..MAX_BATCH {
        let mut sender_pid = 0u32;
        let n = mk_ipc_recv_from(
            SERVICE_INBOX,
            rx.as_mut_ptr(),
            rx.len(),
            RECV_NOWAIT,
            &mut sender_pid,
        );
        if n <= 0 || sender_pid == 0 {
            return;
        }
        let (req, body) = match parse(&rx[..n as usize]) {
            Ok(parsed) => parsed,
            Err((code, req)) => {
                if respond::status(sender_pid, &req, code, tx).is_err() {}
                continue;
            }
        };
        if dispatch(ctx, sender_pid, req, body, tx).is_err() {
            return;
        }
    }
}
