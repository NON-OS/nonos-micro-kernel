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

use super::{tlb::flush_tlb_pcid, types::MAX_PCID};
use core::sync::atomic::{AtomicBool, Ordering};
use spin::Mutex;

pub const KERNEL_PCID: u16 = 0;

static PCID_BITMAP: Mutex<[u64; 64]> = Mutex::new([0; 64]);
static PCID_ENABLED: AtomicBool = AtomicBool::new(false);

pub fn pcid_enabled() -> bool {
    PCID_ENABLED.load(Ordering::Relaxed)
}

pub fn enable_pcid() {
    if !crate::arch::paging::enable_tagged_invalidation() {
        return;
    }
    PCID_ENABLED.store(true, Ordering::SeqCst);
    crate::log::info!("[ADDR_SPACE] PCID enabled");
}

pub fn allocate_pcid() -> u16 {
    let mut bitmap = PCID_BITMAP.lock();

    for i in 1..MAX_PCID {
        let word_idx = (i / 64) as usize;
        let bit_idx = i % 64;

        if bitmap[word_idx] & (1u64 << bit_idx) == 0 {
            bitmap[word_idx] |= 1u64 << bit_idx;
            return i;
        }
    }

    crate::log::log_warning!("[ADDR_SPACE] PCID exhausted, sharing with kernel");
    0
}

pub fn release_pcid(pcid: u16) {
    if pcid == 0 {
        return;
    }

    let mut bitmap = PCID_BITMAP.lock();
    let word_idx = (pcid / 64) as usize;
    let bit_idx = pcid % 64;
    bitmap[word_idx] &= !(1u64 << bit_idx);

    flush_tlb_pcid(pcid);
}
