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

//! Per-vector IRQ slots, written in syscall context, read in hard
//! IRQ context. Each slot is purely atomic so the dispatcher does
//! not have to lock anything to deliver.
//!
//! The bitmap is `AtomicU64` because the broker pool is 64 vectors
//! wide; the previous `AtomicU32` would have rejected (or worse,
//! invoked undefined shift behaviour on) any allocation past slot
//! 31. MSI-X requests use `try_alloc_contiguous` for runs of N
//! consecutive slots so the kernel can hand the device a packed
//! range of vectors.

use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};

use crate::arch::interrupt::broker::BROKER_VEC_COUNT;

pub(super) struct IrqSlot {
    pub active: AtomicBool,
    pub seq: AtomicU64,
    pub overflow: AtomicU64,
    pub grant_id: AtomicU64,
    pub gsi: AtomicU32,
    pub waiter: AtomicU32,
}

impl IrqSlot {
    const fn new() -> Self {
        Self {
            active: AtomicBool::new(false),
            seq: AtomicU64::new(0),
            overflow: AtomicU64::new(0),
            grant_id: AtomicU64::new(0),
            gsi: AtomicU32::new(0),
            waiter: AtomicU32::new(0),
        }
    }
}

const SLOT_INIT: IrqSlot = IrqSlot::new();
pub(super) static SLOTS: [IrqSlot; BROKER_VEC_COUNT] = [SLOT_INIT; BROKER_VEC_COUNT];

// One bit per broker vector. The pool is sized so a `u64` is always
// exactly enough; the const-assert keeps that honest.
const _: () = assert!(BROKER_VEC_COUNT <= 64);
static SLOT_BITMAP: AtomicU64 = AtomicU64::new(0);

#[inline]
fn mask_for(slot: usize) -> u64 {
    1u64 << slot
}

#[inline]
fn run_mask(base: usize, n: usize) -> u64 {
    if n == 64 && base == 0 {
        u64::MAX
    } else {
        ((1u64 << n) - 1) << base
    }
}

pub(super) fn try_alloc_slot() -> Option<usize> {
    loop {
        let cur = SLOT_BITMAP.load(Ordering::Acquire);
        let mut idx = None;
        for i in 0..BROKER_VEC_COUNT {
            if cur & mask_for(i) == 0 {
                idx = Some(i);
                break;
            }
        }
        let i = idx?;
        let new = cur | mask_for(i);
        if SLOT_BITMAP.compare_exchange(cur, new, Ordering::AcqRel, Ordering::Acquire).is_ok() {
            return Some(i);
        }
    }
}

// Reserve `count` consecutive free slots and return the base index.
// Returns `None` if the count is zero, exceeds the pool, or no run
// of that length is currently free. CAS-retried so concurrent
// callers do not corrupt the bitmap.
pub(super) fn try_alloc_contiguous(count: usize) -> Option<usize> {
    if count == 0 || count > BROKER_VEC_COUNT {
        return None;
    }
    loop {
        let cur = SLOT_BITMAP.load(Ordering::Acquire);
        let mut found = None;
        let mut base = 0usize;
        while base + count <= BROKER_VEC_COUNT {
            let mask = run_mask(base, count);
            if cur & mask == 0 {
                found = Some(base);
                break;
            }
            base += 1;
        }
        let b = found?;
        let new = cur | run_mask(b, count);
        if SLOT_BITMAP.compare_exchange(cur, new, Ordering::AcqRel, Ordering::Acquire).is_ok() {
            return Some(b);
        }
    }
}

pub(super) fn free_slot(idx: usize) {
    SLOT_BITMAP.fetch_and(!mask_for(idx), Ordering::AcqRel);
}

pub(super) fn free_contiguous(base: usize, count: usize) {
    if count == 0 || base + count > BROKER_VEC_COUNT {
        return;
    }
    SLOT_BITMAP.fetch_and(!run_mask(base, count), Ordering::AcqRel);
}

pub(super) fn activate(slot_idx: usize, grant_id: u64, gsi: u32) {
    let slot = &SLOTS[slot_idx];
    slot.grant_id.store(grant_id, Ordering::Relaxed);
    slot.gsi.store(gsi, Ordering::Relaxed);
    slot.seq.store(0, Ordering::Relaxed);
    slot.overflow.store(0, Ordering::Relaxed);
    slot.active.store(true, Ordering::Release);
}

pub(super) fn deactivate(slot_idx: usize) {
    let slot = &SLOTS[slot_idx];
    slot.active.store(false, Ordering::Release);
    slot.grant_id.store(0, Ordering::Relaxed);
    slot.gsi.store(0, Ordering::Relaxed);
    slot.waiter.store(0, Ordering::Release);
}

pub(super) fn arm_waiter(slot_idx: usize, pid: u32) {
    SLOTS[slot_idx].waiter.store(pid, Ordering::Release);
}

pub(super) fn disarm_waiter(slot_idx: usize, pid: u32) {
    let _ = SLOTS[slot_idx].waiter.compare_exchange(pid, 0, Ordering::AcqRel, Ordering::Acquire);
}

pub(super) fn read_counters(slot_idx: usize) -> (u64, u64) {
    let slot = &SLOTS[slot_idx];
    (slot.seq.load(Ordering::Acquire), slot.overflow.load(Ordering::Acquire))
}
