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

use crate::protocol::dns::MAGIC_NDNS;
use crate::protocol::errno::E_BAD_MAGIC;
use crate::protocol::ops::MAGIC_NDHC;
use crate::protocol::tcp::MAGIC_NTCP;
use crate::protocol::udp::MAGIC_NUDP;
use crate::server::handlers::health::{handle as health_handle, OP_HEALTHCHECK};
use crate::server::parse_req::{parse, HDR_LEN, IPC_BUF_MAX};
use crate::server::respond::reply;

const SERVICE_INBOX: u64 = 0;
const POLL_MS: u64 = 50;

pub fn run() -> ! {
    let mut rx = vec![0u8; HDR_LEN + IPC_BUF_MAX];
    let mut tx = vec![0u8; HDR_LEN + IPC_BUF_MAX];
    loop {
        crate::iface::poll::pump();
        let mut sender_pid = 0u32;
        let n = mk_ipc_recv_from(
            SERVICE_INBOX,
            rx.as_mut_ptr(),
            rx.len(),
            POLL_MS,
            &mut sender_pid,
        );
        if n <= 0 || sender_pid == 0 {
            continue;
        }
        let Ok((req, body)) = parse(&rx[..n as usize]) else { continue };
        match req.op {
            OP_HEALTHCHECK => health_handle(sender_pid, &req, &mut tx),
            _ => match req.magic {
                MAGIC_NDHC => {
                    crate::server::handlers::dhcp_status::dispatch(sender_pid, &req, &mut tx);
                }
                MAGIC_NTCP => {
                    crate::server::handlers::tcp::dispatch(sender_pid, &req, body, &mut tx);
                }
                MAGIC_NUDP => {
                    crate::server::handlers::udp::dispatch(sender_pid, &req, body, &mut tx);
                }
                MAGIC_NDNS => {
                    crate::server::handlers::dns::dispatch(sender_pid, &req, body, &mut tx);
                }
                _ => {
                    let _ = reply(
                        sender_pid,
                        req.magic,
                        req.op,
                        E_BAD_MAGIC,
                        req.request_id,
                        &[],
                        &mut tx,
                    );
                }
            },
        }
    }
}
