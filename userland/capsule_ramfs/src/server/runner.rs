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

use nonos_libc::{mk_ipc_recv_from, mk_ipc_send};

use super::dispatch::dispatch;
use crate::handles::HandleTable;
use crate::protocol::{decode_request, KERNEL_REPLY_ENDPOINT};
use crate::store::Store;

const MAX_MSG: usize = 8192;

pub fn run() -> ! {
    let mut buf = vec![0u8; MAX_MSG];
    let mut store = Store::new();
    let mut handles = HandleTable::new();
    loop {
        let mut sender_pid: u32 = 0;
        let n = mk_ipc_recv_from(0, buf.as_mut_ptr(), MAX_MSG, 0, &mut sender_pid);
        if n <= 0 {
            continue;
        }
        let req = match decode_request(&buf[..n as usize]) {
            Some(r) => r,
            None => continue,
        };
        let resp = dispatch(&mut store, &mut handles, req, sender_pid);
        let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, resp.as_ptr(), resp.len());
    }
}
