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

use core::sync::atomic::Ordering;

use crate::sys::serial::{print, print_hex};

use super::storage::ring_ref;
use super::types::{DbgEntry, MAGIC, RING_LEN, RING_MASK};

pub fn dbg_drain_to_serial() {
    let ring = ring_ref();
    if ring.magic != MAGIC {
        return;
    }
    let head = ring.head.load(Ordering::Acquire);
    let count = head.min(RING_LEN as u32);
    let start = head - count;
    print(b"\r\n[DBG-RING] drain begin head=");
    print_hex(head as u64);
    print(b"\r\n");
    for i in start..head {
        emit_one(&ring.entries[(i & RING_MASK) as usize]);
    }
    print(b"[DBG-RING] drain end\r\n");
}

fn emit_one(entry: &DbgEntry) {
    print(b"DBG ");
    print_hex(entry.tag as u64);
    print(b" ts=");
    print_hex(entry.ts as u64);
    print(b" cpu=");
    print_hex(entry.cpu as u64);
    print(b" pid=");
    print_hex(entry.pid as u64);
    print(b" pl=");
    for byte in entry.payload.iter() {
        print_hex_byte(*byte);
    }
    print(b"\r\n");
}

fn print_hex_byte(b: u8) {
    const HEX: &[u8] = b"0123456789ABCDEF";
    let hi = HEX[(b >> 4) as usize];
    let lo = HEX[(b & 0x0F) as usize];
    print(&[hi, lo]);
}
