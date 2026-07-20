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

//! Kani proof harnesses for the attestation parser. A model checker proves these
//! over ALL inputs (bounded), not sampled cases. Compiled only under Kani, so
//! ordinary builds and `cargo test` ignore this module. The parser reads only
//! from an untrusted trailer, so its totality (no panic, no out-of-bounds, no
//! overflow, for any byte sequence) is what keeps the spawn gate safe.

use crate::crypto::stark::air::deserialize_proof_ext;

// The proof-ext deserializer must never panic or execute UB for any byte
// sequence: it is the first thing that touches an attacker-controlled trailer.
// Twenty-four bytes exercise the header reads and the bounds arithmetic in
// `take` (including the checked add) before the digest reads fail closed.
#[kani::proof]
#[kani::unwind(3)]
fn proof_deserialize_proof_ext_is_total() {
    let data: [u8; 24] = kani::any();
    let len: usize = kani::any();
    kani::assume(len <= 24);
    let _ = deserialize_proof_ext(&data[..len]);
}

// The empty and one-byte trailers are the degenerate inputs a hostile capsule
// can ship; the parser must refuse them without touching memory it does not own.
#[kani::proof]
fn proof_deserialize_short_trailer_is_total() {
    let data: [u8; 1] = kani::any();
    let len: usize = kani::any();
    kani::assume(len <= 1);
    let _ = deserialize_proof_ext(&data[..len]);
}
