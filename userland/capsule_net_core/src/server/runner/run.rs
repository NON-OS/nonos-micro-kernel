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

use crate::server::parse_req::{parse, HDR_LEN, IPC_BUF_MAX};
use crate::server::runner::{dispatch, receive};

pub fn run() -> ! {
    let mut rx = vec![0u8; HDR_LEN + IPC_BUF_MAX];
    let mut tx = vec![0u8; HDR_LEN + IPC_BUF_MAX];
    loop {
        crate::iface::poll::pump();
        let mut sender_pid = 0u32;
        let n = receive::receive(&mut rx, &mut sender_pid);
        if n <= 0 || sender_pid == 0 {
            continue;
        }
        let Ok((req, body)) = parse(&rx[..n as usize]) else {
            continue;
        };
        dispatch::dispatch(sender_pid, &req, body, &mut tx);
    }
}
