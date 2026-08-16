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

//! One attestation for a whole capsule set, and the bytes carrying it.
//!
//! A trailer per capsule costs a proof per capsule to make and a proof-sized
//! blob per capsule to ship. Every capsule opens the same policy tree, so one
//! shared-root proof covers the set: the kernel verifies once at boot and each
//! spawn is then a lookup against a table the proof has already bound.
//!
//! Layout, all integers big-endian:
//!
//! ```text
//!   magic   8   "NZKAGGR1"
//!   epoch   8   policy epoch
//!   count   2   enrolled capsules
//!   depth   1   policy tree depth
//!   entry   count * (32 measurement + 8 capabilities)
//!   paths   count * depth * 32
//!   proof   the shared-root STARK over the whole batch
//! ```
//!
//! Directions are not carried: an entry's index is its position, so the verifier
//! derives them rather than trusting them. Siblings are carried but need no
//! trust either, since a path built from wrong ones does not fold to the root.

use super::super::field::{Fp, P};
use super::poseidon::RATE;
use alloc::vec::Vec;

pub const AGGREGATE_MAGIC: &[u8; 8] = b"NZKAGGR1";

/// Bytes before the entry table.
pub const HEADER: usize = 8 + 8 + 2 + 1;
/// Bytes per entry in the table.
pub const ENTRY: usize = 32 + 8;
/// Bytes per sibling digest.
pub const NODE: usize = 32;

/// What the proof binds for one capsule: what it is and what it may do.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Entry {
    pub measurement: [u8; 32],
    pub caps: u64,
}

/// A rate-width digest as the gate serializes one, each lane little-endian.
pub fn digest_to_bytes(d: [Fp; RATE]) -> [u8; 32] {
    let mut out = [0u8; 32];
    for (i, lane) in d.iter().enumerate() {
        out[i * 8..i * 8 + 8].copy_from_slice(&lane.value().to_le_bytes());
    }
    out
}

/// The inverse, refusing any word outside the field rather than reducing it, so
/// two different byte strings can never name the same digest.
pub fn digest_from_bytes(b: &[u8]) -> Option<[Fp; RATE]> {
    if b.len() < 32 {
        return None;
    }
    let mut out = [Fp::ZERO; RATE];
    for (i, lane) in out.iter_mut().enumerate() {
        let mut w = [0u8; 8];
        w.copy_from_slice(&b[i * 8..i * 8 + 8]);
        let v = u64::from_le_bytes(w);
        if v >= P {
            return None;
        }
        *lane = Fp::from_u64(v);
    }
    Some(out)
}

/// The context the aggregate proof is drawn under: the epoch and the whole
/// table. Binding every measurement and capability word into the transcript is
/// what stops an entry being edited after the fact, since the table is the
/// statement rather than something carried beside it.
pub fn table_context(epoch: u64, entries: &[Entry]) -> Vec<u8> {
    let mut ctx = Vec::with_capacity(26 + entries.len() * ENTRY);
    ctx.extend_from_slice(b"NONOS-AGGREGATE-v1");
    ctx.extend_from_slice(&epoch.to_be_bytes());
    for e in entries {
        ctx.extend_from_slice(&e.measurement);
        ctx.extend_from_slice(&e.caps.to_be_bytes());
    }
    ctx
}

/// Where each section starts, or `None` if the blob cannot hold what its header
/// claims. Total over any bytes: every later read is inside a checked length.
pub struct Layout {
    pub epoch: u64,
    pub count: usize,
    pub depth: usize,
    pub entries_at: usize,
    pub paths_at: usize,
    pub proof_at: usize,
}

pub fn parse_header(blob: &[u8]) -> Option<Layout> {
    if blob.len() < HEADER || &blob[0..8] != AGGREGATE_MAGIC {
        return None;
    }
    let mut e = [0u8; 8];
    e.copy_from_slice(&blob[8..16]);
    let epoch = u64::from_be_bytes(e);
    let count = u16::from_be_bytes([blob[16], blob[17]]) as usize;
    let depth = blob[18] as usize;
    if count == 0 || depth == 0 {
        return None;
    }
    let entries_at = HEADER;
    let paths_at = entries_at.checked_add(count.checked_mul(ENTRY)?)?;
    let proof_at = paths_at.checked_add(count.checked_mul(depth.checked_mul(NODE)?)?)?;
    if blob.len() < proof_at {
        return None;
    }
    Some(Layout { epoch, count, depth, entries_at, paths_at, proof_at })
}

/// The entry table, or `None` if any word is malformed.
pub fn parse_entries(blob: &[u8], l: &Layout) -> Option<Vec<Entry>> {
    let mut out = Vec::with_capacity(l.count);
    for i in 0..l.count {
        let at = l.entries_at + i * ENTRY;
        let mut measurement = [0u8; 32];
        measurement.copy_from_slice(&blob[at..at + 32]);
        let mut c = [0u8; 8];
        c.copy_from_slice(&blob[at + 32..at + 40]);
        out.push(Entry { measurement, caps: u64::from_be_bytes(c) });
    }
    Some(out)
}
