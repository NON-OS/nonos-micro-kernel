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

use nonos_libc::{mk_ipc_recv, mk_ipc_send, mk_yield};

use super::handlers::route;
use crate::protocol::IPC_PAYLOAD_MAX;
use crate::state::PowerState;

const SERVICE_PORT: u32 = 4448;

pub fn run() -> ! {
    let mut in_buf = vec![0u8; IPC_PAYLOAD_MAX];
    let mut out_buf = vec![0u8; IPC_PAYLOAD_MAX];
    let mut state = PowerState::new();
    loop {
        let received = mk_ipc_recv(SERVICE_PORT, in_buf.as_mut_ptr(), in_buf.len());
        if received <= 0 {
            mk_yield();
            continue;
        }
        let n = route(&mut state, &in_buf[..received as usize], &mut out_buf);
        if n > 0 {
            let _ = mk_ipc_send(SERVICE_PORT, out_buf.as_ptr(), n);
        }
    }
}
