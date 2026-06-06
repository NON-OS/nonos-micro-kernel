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
use crate::state::Context;

use super::constants::{RECV_TIMEOUT_MS, SERVICE_INBOX, SWEEP_INTERVAL_TICKS};
use super::dispatch::dispatch;
use super::sweep_dead::sweep_dead;

pub fn run(mut ctx: Context) -> ! {
    let mut rx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut tx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut sweep_ticks = 0u32;
    loop {
        sweep_ticks = sweep_ticks.wrapping_add(1);
        if sweep_ticks % SWEEP_INTERVAL_TICKS == 0 {
            sweep_dead(&mut ctx);
        }
        let mut sender_pid = 0u32;
        let n = mk_ipc_recv_from(
            SERVICE_INBOX,
            rx.as_mut_ptr(),
            rx.len(),
            RECV_TIMEOUT_MS,
            &mut sender_pid,
        );
        if n <= 0 || sender_pid == 0 {
            continue;
        }
        let Some((req, body)) = parse(&rx[..n as usize]) else { continue };
        dispatch(&mut ctx, sender_pid, req, body, &mut tx);
    }
}
