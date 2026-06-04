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

use alloc::vec;

use nonos_libc::mk_ipc_recv_from;

use crate::protocol::{parse, HDR_LEN, IPC_PAYLOAD_MAX};
use crate::server::respond;
use crate::state::Context;

const SERVICE_INBOX: u64 = 0;
const RECV_BLOCK: u64 = 0;
const RECV_RETRY_MS: u64 = 16;

pub fn run(mut ctx: Context) -> ! {
    super::paint_initial::paint_initial(&mut ctx);
    let mut rx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut tx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    loop {
        let mut sender_pid = 0u32;
        let timeout =
            if super::ready_to_block::ready_to_block(&ctx) { RECV_BLOCK } else { RECV_RETRY_MS };
        let n =
            mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), timeout, &mut sender_pid);
        super::retry_input_subscription::retry_input_subscription(&mut ctx);
        super::retry_wm_subscription::retry_wm_subscription(&mut ctx);
        if n <= 0 || sender_pid == 0 {
            continue;
        }
        if super::wm_notify::handle(&mut ctx, &rx[..n as usize]) {
            continue;
        }
        if super::input::handle(&mut ctx, &rx[..n as usize]) {
            continue;
        }
        let (req, body) = match parse(&rx[..n as usize]) {
            Ok(parsed) => parsed,
            Err((code, req)) => {
                let _ = respond::status(sender_pid, &req, code, &mut tx);
                continue;
            }
        };
        super::dispatch::dispatch(&mut ctx, sender_pid, req, body, &mut tx);
    }
}
