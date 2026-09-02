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

//! The interrupt lines the kernel routes to itself.
//!
//! A capsule binding INTx is checked against the grant records, which cover
//! every line another capsule holds. They do not cover the kernel's own: the
//! keyboard and mouse redirection entries are programmed through
//! `sys::apic::ioapic`, a different module from the one the broker routes
//! with, so nothing in `records` ever describes them.
//!
//! That left a capsule whose device reports one of those lines free to
//! reprogram the entry and point the kernel's own interrupt at a vector it
//! owns. Firmware assigns the Interrupt Line register and the config-write
//! allowlist refuses to let a capsule change it, so this is not a line a
//! driver can pick at will, but "unlikely to be chosen" is not a boundary.
//! A line the kernel is listening on is not available, and the answer says so
//! rather than reporting the collision as somebody else's grant.

use core::sync::atomic::{AtomicU64, Ordering};

/// GSIs 0..255, which is every line an IOAPIC redirection table can carry.
const WORDS: usize = 4;

static RESERVED: [AtomicU64; WORDS] =
    [AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0)];

/// Record that the kernel has taken `gsi` for itself. Called from wherever a
/// redirection entry is programmed on the kernel's behalf, before interrupts
/// are unmasked, so no capsule can bind it in between.
pub fn reserve(gsi: u32) {
    if let Some((word, bit)) = index(gsi) {
        RESERVED[word].fetch_or(1u64 << bit, Ordering::Release);
    }
}

/// Whether `gsi` is one the kernel routes to itself.
pub fn is_reserved(gsi: u32) -> bool {
    match index(gsi) {
        Some((word, bit)) => RESERVED[word].load(Ordering::Acquire) & (1u64 << bit) != 0,
        // Out of range for any redirection table. Treated as reserved so a
        // malformed line is refused rather than programmed.
        None => true,
    }
}

fn index(gsi: u32) -> Option<(usize, u32)> {
    let g = gsi as usize;
    if g >= WORDS * 64 {
        return None;
    }
    Some((g / 64, (g % 64) as u32))
}
