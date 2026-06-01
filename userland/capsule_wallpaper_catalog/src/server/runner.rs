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

use crate::protocol::{Header, E_INVAL, HDR_LEN, IPC_PAYLOAD_MAX, OP_GET_CHUNK, OP_GET_COUNT, OP_GET_SIZE, OP_GET_SLUG};

use super::handlers::{op_get_chunk, op_get_count, op_get_size, op_get_slug};
use super::{recv, respond};

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
        let hdr = match Header::decode(&buf[..HDR_LEN]) {
            Some(h) => h,
            None => continue,
        };
        match hdr.op {
            OP_GET_COUNT => op_get_count::handle(sender),
            OP_GET_SIZE => op_get_size::handle(sender, hdr.index),
            OP_GET_CHUNK => op_get_chunk::handle(sender, hdr.index, hdr.offset),
            OP_GET_SLUG => op_get_slug::handle(sender, hdr.index),
            _ => respond::err(sender, hdr.op, hdr.index, E_INVAL),
        }
    }
}
