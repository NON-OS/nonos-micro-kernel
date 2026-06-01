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

use core::sync::atomic::{compiler_fence, Ordering};

use super::storage::ring_mut;
use super::types::{MAGIC, RING_MASK};

#[inline(never)]
pub fn dbg_emit(tag: u32, payload: &[u8]) {
    let ring = ring_mut();
    if ring.magic != MAGIC {
        ring.magic = MAGIC;
    }
    let idx = (ring.head.fetch_add(1, Ordering::Relaxed) & RING_MASK) as usize;
    let entry = &mut ring.entries[idx];
    entry.tag = tag;
    entry.ts = read_tsc_lo();
    entry.cpu = 0;
    entry.pid = 0;
    let n = payload.len().min(16);
    entry.payload[..n].copy_from_slice(&payload[..n]);
    for byte in &mut entry.payload[n..] {
        *byte = 0;
    }
    compiler_fence(Ordering::Release);
}

#[inline(never)]
pub fn dbg_emit_u64(tag: u32, v: u64) {
    dbg_emit(tag, &v.to_le_bytes());
}

#[inline(never)]
pub fn dbg_emit_2u64(tag: u32, a: u64, b: u64) {
    let mut buf = [0u8; 16];
    buf[..8].copy_from_slice(&a.to_le_bytes());
    buf[8..].copy_from_slice(&b.to_le_bytes());
    dbg_emit(tag, &buf);
}

#[inline(always)]
fn read_tsc_lo() -> u32 {
    let lo: u32;
    unsafe {
        core::arch::asm!("rdtsc", out("eax") lo, out("edx") _, options(nomem, nostack));
    }
    lo
}
