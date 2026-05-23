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

//! Free-quarantine ring. Freed user regions are filled with POISON_BYTE and
//! held so reuse is delayed; on eviction the region is re-checked and any byte
//! that is no longer poison means something wrote to freed memory (the word is
//! logged, revealing the stray writer's value).

use spin::Mutex;

// Canonical (bit 47 = 0) but unmapped (~104 TiB), so a use-after-free *read* of
// a poisoned pointer faults as a #PF with cr2 = this value (visible in the trap
// dump), while a stray *write* is caught by the eviction re-check below.
const POISON_WORD: u64 = 0x0000_5EED_5EED_5EED;
const SLOTS: usize = 1024;
const SCAN_CAP: usize = 4096;

pub fn poison(user: usize, ulen: usize) {
    let words = ulen / 8;
    for i in 0..words {
        unsafe { core::ptr::write_volatile((user + i * 8) as *mut u64, POISON_WORD) };
    }
}

#[derive(Clone, Copy)]
struct Slot {
    raw: usize,
    size: usize,
    align: usize,
    user: usize,
    ulen: usize,
}

struct Ring {
    buf: [Slot; SLOTS],
    head: usize,
    len: usize,
}

static RING: Mutex<Ring> = Mutex::new(Ring {
    buf: [Slot { raw: 0, size: 0, align: 0, user: 0, ulen: 0 }; SLOTS],
    head: 0,
    len: 0,
});

pub fn push(
    raw: usize,
    size: usize,
    align: usize,
    user: usize,
    ulen: usize,
) -> Option<(usize, usize, usize)> {
    let mut r = RING.lock();
    let evicted = if r.len == SLOTS {
        let s = r.buf[r.head];
        r.head = (r.head + 1) % SLOTS;
        r.len -= 1;
        check_poison(s.user, s.ulen);
        Some((s.raw, s.size, s.align))
    } else {
        None
    };
    let idx = (r.head + r.len) % SLOTS;
    r.buf[idx] = Slot { raw, size, align, user, ulen };
    r.len += 1;
    evicted
}

fn check_poison(user: usize, ulen: usize) {
    use core::sync::atomic::{AtomicU32, Ordering};
    static N: AtomicU32 = AtomicU32::new(0);
    let words = core::cmp::min(ulen, SCAN_CAP) / 8;
    for i in 0..words {
        let w = unsafe { core::ptr::read_volatile((user + i * 8) as *const u64) };
        if w != POISON_WORD {
            if N.fetch_add(1, Ordering::Relaxed) < 40 {
                crate::sys::serial::print(b"[UAF-WRITE] user=");
                crate::arch::x86_64::diag::print_hex_u64(user as u64);
                crate::sys::serial::print(b" off=");
                crate::arch::x86_64::diag::print_hex_u64((i * 8) as u64);
                crate::sys::serial::print(b" word=");
                crate::arch::x86_64::diag::print_hex_u64(w);
                crate::sys::serial::println(b"");
            }
            return;
        }
    }
}
