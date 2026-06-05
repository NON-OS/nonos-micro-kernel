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

use super::constants::{RECV_BLOCK, RECV_RETRY_MS, SERVICE_INBOX};
use crate::protocol::parse;
use crate::server::respond;
use crate::state::Context;

pub(super) fn drain(ctx: &mut Context, rx: &mut [u8], tx: &mut [u8]) {
    loop {
        let mut sender_pid = 0u32;
        let timeout = if crate::server::ready_to_block::ready_to_block(ctx) {
            RECV_BLOCK
        } else {
            RECV_RETRY_MS
        };
        let n =
            mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), timeout, &mut sender_pid);
        crate::server::retry_input_subscription::retry_input_subscription(ctx);
        crate::server::retry_wm_subscription::retry_wm_subscription(ctx);
        if n <= 0 || sender_pid == 0 {
            return;
        }
        if crate::server::wm_notify::handle(ctx, &rx[..n as usize]) {
            continue;
        }
        if crate::server::input::handle(ctx, &rx[..n as usize]) {
            continue;
        }
        let (req, body) = match parse(&rx[..n as usize]) {
            Ok(parsed) => parsed,
            Err((code, req)) => {
                let _ = respond::status(sender_pid, &req, code, tx);
                continue;
            }
        };
        crate::server::dispatch::dispatch(ctx, sender_pid, req, body, tx);
    }
}
