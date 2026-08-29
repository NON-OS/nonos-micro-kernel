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

use nonos_libc::{mk_debug, mk_ipc_recv_from, mk_ipc_send};

use super::dispatch::dispatch;
use super::seeder::PackageSeeder;
use crate::protocol::{decode_request, encode_response, EINVAL, KERNEL_REPLY_ENDPOINT};
use crate::store::Store;

const MAX_MSG: usize = 65556;

pub fn run() -> ! {
    let mut buf = vec![0u8; MAX_MSG];
    let mut store = Store::new();
    store.seed();
    let mut seeder = PackageSeeder::new();
    let ready = b"[VFSD] loop\n";
    let _ = mk_debug(ready.as_ptr(), ready.len());
    loop {
        let mut sender_pid: u32 = 0;
        let n = mk_ipc_recv_from(0, buf.as_mut_ptr(), MAX_MSG, seeder.poll_ms(), &mut sender_pid);
        if n <= 0 {
            seeder.on_idle(&mut store);
            continue;
        }
        seeder.saw_request();
        let n = n as usize;
        let resp = match decode_request(&buf[..n]) {
            Ok(req) => dispatch(&mut store, req, sender_pid),
            Err(_) => encode_response(0, 0, 0, EINVAL, &[]),
        };
        let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, resp.as_ptr(), resp.len());
    }
}
