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

// Shared layout contract with the proving side (host tools). Both sides must
// agree on every constant here. It matches the proving side's public-input
// encoding (generate_proof_support/public_inputs.rs).
//
// Seven public input field elements, each 32-byte big-endian. A 256-bit hash
// does not fit in one BLS12-381 field element, so hashes are split high/low
// across two elements, each carrying 16 value bytes right-aligned (the high 16
// bytes of the element are zero):
//
//   fe[0] capsule_hash[0..16]
//   fe[1] capsule_hash[16..32]
//   fe[2] program_hash[0..16]
//   fe[3] program_hash[16..32]
//   fe[4] capability_mask   u64, low 8 bytes
//   fe[5] commitment[0..16]
//   fe[6] commitment[16..32]
//
// The commitment is a domain-separated hash of the base field elements,
// produced by the prover and bound into the proof. The kernel does NOT recompute
// it, so it stays independent of the commitment hash the proving side uses. The
// security the kernel enforces is the binding below: capsule_hash must equal
// blake3 of the real bytes, and the capability mask must equal the grant. The
// commitment is checked only for trailer integrity against the public input.

pub(super) const FE: usize = 32;
pub(super) const PI_COUNT: usize = 7;

pub(super) const FI_CAPSULE_HASH_HI: usize = 0;
pub(super) const FI_CAP_MASK: usize = 4;
pub(super) const FI_COMMITMENT_HI: usize = 5;

// Copy a single 32-byte field element out of the flat public-input buffer.
// Returns None when the index is out of range (never panics).
pub(super) fn fe(pubins: &[u8], idx: usize) -> Option<[u8; 32]> {
    let slice = pubins.get(idx * FE..idx * FE + FE)?;
    let mut out = [0u8; 32];
    out.copy_from_slice(slice);
    Some(out)
}

// Reconstruct a 32-byte value from two consecutive field elements holding its
// high and low 16 bytes, each right-aligned in a big-endian field element.
pub(super) fn join_hi_lo(pubins: &[u8], hi_idx: usize) -> Option<[u8; 32]> {
    let hi = fe(pubins, hi_idx)?;
    let lo = fe(pubins, hi_idx + 1)?;
    let mut out = [0u8; 32];
    out[0..16].copy_from_slice(&hi[16..32]);
    out[16..32].copy_from_slice(&lo[16..32]);
    Some(out)
}
