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
use crate::protocol::{decode_request, encode_response, EINVAL, KERNEL_REPLY_ENDPOINT};
use crate::store::Store;

const MAX_MSG: usize = 65556;

pub fn run() -> ! {
    let mut buf = vec![0u8; MAX_MSG];
    let mut store = Store::new();
    store.seed();
    // Seeding blocks on the package store, up to one blk timeout per call,
    // and a failure leaves /capsules silently empty. One line says how the
    // seed ended, so a missing desktop names its layer instead of reading as
    // a shell timeout two layers up.
    let status = crate::blk::status::current();
    let mut line = *b"[VFS] serving, store status 00";
    let n = line.len();
    line[n - 2] = b'0' + ((status / 10) % 10) as u8;
    line[n - 1] = b'0' + (status % 10) as u8;
    let _ = mk_debug(line.as_ptr(), n);
    loop {
        let mut sender_pid: u32 = 0;
        let n = mk_ipc_recv_from(0, buf.as_mut_ptr(), MAX_MSG, 0, &mut sender_pid);
        if n <= 0 {
            continue;
        }
        let n = n as usize;
        let started = nonos_libc::mk_uptime_ms();
        let op = if n >= 8 { u16::from_le_bytes([buf[6], buf[7]]) } else { 0 };
        let resp = match decode_request(&buf[..n]) {
            Ok(req) => dispatch(&mut store, req, sender_pid),
            Err(_) => encode_response(0, 0, 0, EINVAL, &[]),
        };
        let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, resp.as_ptr(), resp.len());
        // A handler that outlives its caller's timeout turns every reply into
        // a drop and reads as a dead service. Name the op and the cost.
        let spent = nonos_libc::mk_uptime_ms().saturating_sub(started);
        if spent > 1000 {
            let mut line = *b"[VFS] slow op 0000 ms 000000";
            for (i, shift) in [(14usize, 12u32), (15, 8), (16, 4), (17, 0)] {
                line[i] = b"0123456789abcdef"[((op as usize) >> shift) & 0xF];
            }
            let ms = spent.min(999_999) as u32;
            let mut v = ms;
            for i in (22..28).rev() {
                line[i] = b'0' + (v % 10) as u8;
                v /= 10;
            }
            let _ = mk_debug(line.as_ptr(), line.len());
        }
    }
}
