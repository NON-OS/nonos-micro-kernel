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

use nonos_libc::mk_yield;

use nonos_policy_proto::{decode_field, Header, E_INVAL, HDR_LEN, IPC_PAYLOAD_MAX, OP_GET, OP_SET};

use super::{handle_get, handle_set, recv, respond};

pub fn run(endpoint: u64) -> ! {
    let mut buf = [0u8; IPC_PAYLOAD_MAX];
    let mut sender: u32 = 0;
    loop {
        let n = recv::poll(endpoint, &mut buf, &mut sender as *mut u32);
        if n <= 0 {
            mk_yield();
            continue;
        }
        if (n as usize) < HDR_LEN {
            continue;
        }
        let total = n as usize;
        let hdr = match Header::decode(&buf[..HDR_LEN]) {
            Some(h) => h,
            None => continue,
        };
        let body_end = HDR_LEN + hdr.payload_len as usize;
        if body_end > total {
            respond::err(sender, hdr.op, hdr.field, hdr.kind, E_INVAL);
            continue;
        }
        let field = match decode_field(hdr.field) {
            Some(f) => f,
            None => {
                respond::err(sender, hdr.op, hdr.field, hdr.kind, E_INVAL);
                continue;
            }
        };
        match hdr.op {
            OP_GET => handle_get::dispatch(sender, field),
            OP_SET => handle_set::dispatch(sender, field, &buf[HDR_LEN..body_end]),
            _ => respond::err(sender, hdr.op, hdr.field, hdr.kind, E_INVAL),
        }
    }
}
