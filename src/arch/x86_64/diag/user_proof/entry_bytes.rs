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

//! Print the first sixteen bytes the CPU will fetch at the user RIP.
//! Reads are routed through the directmap from the leaf's physical
//! base, so the audit never dereferences a user VA. If the loader
//! mis-laid the segment and the page is mapped USER + executable
//! but the bytes are wrong, this is the line that catches it before
//! iretq.

use super::page_walk::Leaf;
use crate::memory::layout::DIRECTMAP_BASE;

const BYTES_PER_LINE: usize = 16;
const HEX: &[u8; 16] = b"0123456789abcdef";

pub(super) fn print(user_rip: u64, leaf: Leaf) {
    let phys = leaf.phys_base + leaf.offset;
    let src = (DIRECTMAP_BASE + phys) as *const u8;
    let mut buf = [0u8; BYTES_PER_LINE];
    // SAFETY: ek@nonos.systems — `leaf` came from the page walker,
    // so `phys_base` is a real physical page covered by the
    // directmap. `offset` is bounded by the leaf granularity. The
    // copy reads at most 16 bytes inside that page.
    unsafe {
        for i in 0..BYTES_PER_LINE {
            buf[i] = core::ptr::read_volatile(src.add(i));
        }
    }
    emit(user_rip, &buf);
}

fn emit(rip: u64, bytes: &[u8]) {
    crate::sys::serial::print(b"[USER-PROOF] bytes rip=");
    super::super::print_hex::print_hex_u64(rip);
    crate::sys::serial::print(b" =");
    let mut text = [b' '; BYTES_PER_LINE * 3];
    for (i, b) in bytes.iter().enumerate() {
        text[i * 3] = b' ';
        text[i * 3 + 1] = HEX[(b >> 4) as usize];
        text[i * 3 + 2] = HEX[(b & 0xf) as usize];
    }
    crate::sys::serial::print(&text);
    crate::sys::serial::println(b"");
}
