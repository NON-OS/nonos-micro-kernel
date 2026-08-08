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

//! The executable specification the differential proofs compare against, each
//! function restating the contract a Lean model formalizes, independent of the
//! implementation.

// Buddy order to size: verification/lean Nonos/Buddy.lean split_conserves. A
// block of order k spans two to the k bytes.
pub fn buddy_order_size(order: usize) -> usize {
    1usize << order
}

// Bitmap index arithmetic: verification/lean Nonos/Bitmap.lean. The byte and
// bit position of an index, restated with a modulo rather than the mask the
// implementation uses.
pub fn bitmap_byte_of(idx: usize) -> usize {
    idx / 8
}

pub fn bitmap_bit_mask(idx: usize) -> u8 {
    1u8 << (idx % 8)
}

// Region range algebra: verification/lean Nonos/Interval.lean and
// Nonos/Vma.lean. Two half-open ranges overlap unless one ends at or before the
// other begins; the two are exact negations.
pub fn region_overlaps(a0: u64, a1: u64, b0: u64, b1: u64) -> bool {
    a0 < b1 && b0 < a1
}

pub fn region_disjoint(a0: u64, a1: u64, b0: u64, b1: u64) -> bool {
    a1 <= b0 || b1 <= a0
}

// Load-balancer elapsed test: verification/lean Nonos/Timer.lean. The interval
// is reached when the elapsed ticks cover it; a wraparound (current before last)
// reads as zero elapsed, restated here with an explicit branch.
pub fn timer_elapsed_reached(current: u64, last: u64, interval: u64) -> bool {
    if current >= last {
        current - last >= interval
    } else {
        interval == 0
    }
}

// Quota check: verification/lean Nonos/Quota.lean. A request is covered when it
// is at most the remaining budget.
pub fn quota_has_at_least(remaining: u64, amount: u64) -> bool {
    amount <= remaining
}

// Ring index wrap: verification/lean Nonos/Ring.lean. A position advances by one
// and wraps at the capacity.
pub fn ring_wrap(pos: usize, cap: usize) -> usize {
    (pos + 1) % cap
}

// MMIO window validity: verification/lean Nonos/Mmio.lean. A window is valid
// when it is non-empty and does not overflow the address space.
pub fn mmio_range_ok(base: usize, size: usize) -> bool {
    size != 0 && base.checked_add(size).is_some()
}

// Reference count decrement: verification/lean Nonos/Refcount.lean. A live count
// lowers by one; a zero count has no predecessor.
pub fn refcount_dec(ref_count: u32) -> Option<u32> {
    if ref_count == 0 {
        None
    } else {
        Some(ref_count - 1)
    }
}

// Relocation-target bounds: verification/lean Nonos/Bounds.lean. An access is in
// range when it sits wholly inside the segment and neither end overflows.
pub fn in_range(addr: u64, size: u64, start: u64, seg_size: u64) -> bool {
    match (start.checked_add(seg_size), addr.checked_add(size)) {
        (Some(end), Some(addr_end)) => start <= addr && addr_end <= end,
        _ => false,
    }
}

// Scheduling priority order: verification/lean Nonos/Priority.lean. Restated
// from the policy numbers directly (1 fifo, 2 rr, 3 batch, 5 idle, 6 deadline),
// independent of the named constants. Higher preempts lower.
pub fn effective_priority(policy: i32, rt_priority: i32, nice: i32) -> i32 {
    if policy == 1 || policy == 2 {
        100 + rt_priority
    } else if policy == 6 {
        200
    } else if policy == 5 {
        -1
    } else if policy == 3 {
        19 - nice
    } else {
        20 - nice
    }
}

// Restored RFLAGS: verification/lean Nonos/Rflags.lean. Restated from the bit
// positions directly, independent of the kernel's mask constant: every bit
// ring 0 controls is cleared, bit 1 is set, and nothing else is touched.
pub fn sanitize_rflags(rflags: u64) -> u64 {
    let privileged: u64 = (1 << 8)
        | (1 << 10)
        | (1 << 12)
        | (1 << 13)
        | (1 << 14)
        | (1 << 16)
        | (1 << 17)
        | (1 << 18)
        | (1 << 19)
        | (1 << 20);
    (rflags & !privileged) | (1 << 1)
}

// Spawn capability ceiling: verification/lean Nonos/SpawnCaps.lean. Restated
// as set containment over the bits, independent of the kernel's expressions.
pub fn spawn_within_ceiling(required: u64, optional: u64, ceiling: u64) -> bool {
    (0..64).all(|b| {
        let asked = (required >> b) & 1 == 1 || (optional >> b) & 1 == 1;
        !asked || (ceiling >> b) & 1 == 1
    })
}

pub fn spawn_grant_within_manifest(required: u64, optional: u64, granted: u64) -> bool {
    (0..64).all(|b| {
        let given = (granted >> b) & 1 == 1;
        !given || (required >> b) & 1 == 1 || (optional >> b) & 1 == 1
    })
}

pub fn spawn_install_caps(required: u64, optional: u64, granted: u64) -> u64 {
    let mut out = 0u64;
    for b in 0..64 {
        let keep =
            (required >> b) & 1 == 1 || ((optional >> b) & 1 == 1 && (granted >> b) & 1 == 1);
        if keep {
            out |= 1 << b;
        }
    }
    out
}

// Delegation expiry: verification/lean Nonos/Delegation.lean. A delegation
// never outlives its parent, and never outlasts what was asked for.
pub fn delegation_expiry(requested: Option<u64>, parent: Option<u64>) -> Option<u64> {
    match (requested, parent) {
        (Some(r), Some(p)) => Some(if r < p { r } else { p }),
        (None, Some(p)) => Some(p),
        (r, None) => r,
    }
}
