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
use nonos_libc::{mk_exit, mk_ipc_recv, mk_ipc_send, mk_time_millis, mk_yield};

use super::handlers::route;
use crate::protocol::{
    parse, DEFAULT_IDLE_TIMEOUT_MS, IPC_PAYLOAD_MAX, MAX_DEPTH, MAX_TOTAL_BYTES,
};
use crate::state::Clipboard;

const SERVICE_PORT: u32 = 4414;

pub fn run() -> ! {
    let now = read_time();
    let mut clipboard = Clipboard::new(MAX_DEPTH, MAX_TOTAL_BYTES, DEFAULT_IDLE_TIMEOUT_MS, now);
    let mut in_buf = vec![0u8; IPC_PAYLOAD_MAX];
    let mut out_buf = vec![0u8; IPC_PAYLOAD_MAX];
    loop {
        clipboard.expire_if_idle(read_time());
        let received = mk_ipc_recv(SERVICE_PORT, in_buf.as_mut_ptr(), in_buf.len());
        if received <= 0 {
            mk_yield();
            continue;
        }
        let now = read_time();
        let payload = &in_buf[..received as usize];
        let n = route(&mut clipboard, payload, &mut out_buf, now);
        if n > 0 {
            let _ = mk_ipc_send(SERVICE_PORT, out_buf.as_ptr(), n);
        }
    }
}

fn read_time() -> u64 {
    let raw = mk_time_millis();
    if raw < 0 {
        return 0;
    }
    raw as u64
}

#[allow(dead_code)]
fn _terminate() -> ! {
    mk_exit(0);
    loop {
        mk_yield();
    }
}
