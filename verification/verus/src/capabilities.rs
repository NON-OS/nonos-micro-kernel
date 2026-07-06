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

//! Machine-checked theorems about the NONOS capability algebra, verified by
//! Verus (SMT). The spec functions below mirror `src/capabilities/bits.rs`
//! exactly: a right is a single power-of-two bit, a token is the bitwise-OR of
//! its granted rights, and the kernel's `has_capability`, `remove_capability`,
//! `add_capability`, and attenuation are the bit operations modelled here. The
//! proofs therefore establish properties of the operations the kernel actually
//! executes, with no separate abstract model to trust.
//!
//! Verify with: verus --crate-type=lib src/lib.rs

use vstd::prelude::*;

verus! {

// bits & cap.bit() != 0: kernel `has_capability`.
pub open spec fn has(bits: u64, bit: u64) -> bool {
    (bits & bit) != 0
}

// bits & !cap.bit(): kernel `remove_capability` (revocation).
pub open spec fn revoke(bits: u64, bit: u64) -> u64 {
    bits & !bit
}

// bits | cap.bit(): kernel `add_capability` (grant).
pub open spec fn grant(bits: u64, bit: u64) -> u64 {
    bits | bit
}

// bits & mask: attenuation of a token by an authority mask.
pub open spec fn attenuate(bits: u64, mask: u64) -> u64 {
    bits & mask
}

/// Revocation is monotonic: a revoked token grants a right only if the original
/// token also granted it. Authority can never appear from a revoke.
pub proof fn revoke_is_monotonic(bits: u64, c: u64, r: u64)
    ensures
        has(revoke(bits, c), r) ==> has(bits, r),
{
    assert(((bits & !c) & r) != 0 ==> (bits & r) != 0) by (bit_vector);
}

/// Revoking a right actually drops it: the result never grants the revoked bit.
pub proof fn revoke_drops_the_right(bits: u64, c: u64)
    ensures
        !(has(revoke(bits, c), c)),
{
    assert(((bits & !c) & c) == 0) by (bit_vector);
}

/// Attenuation confines: an attenuated token can never gain a right the parent
/// token lacked. This is the core object-capability confinement property.
pub proof fn attenuation_confines(bits: u64, mask: u64, r: u64)
    ensures
        !(has(bits, r)) ==> !(has(attenuate(bits, mask), r)),
{
    assert((bits & r) == 0 ==> ((bits & mask) & r) == 0) by (bit_vector);
}

/// Granting is a monotonic increase: it preserves every existing right and adds
/// exactly the requested one. `c != 0` because a right is a real, nonzero bit.
pub proof fn grant_preserves_and_adds(bits: u64, c: u64, r: u64)
    requires
        c != 0,
    ensures
        has(bits, r) ==> has(grant(bits, c), r),
        has(grant(bits, c), c),
{
    assert((bits & r) != 0 ==> ((bits | c) & r) != 0) by (bit_vector);
    assert(c != 0 ==> ((bits | c) & c) != 0) by (bit_vector);
}

/// Unforgeability of the empty authority: a token that grants nothing (all bits
/// zero) grants no right whatsoever, so authority cannot be conjured from an
/// empty token by any query.
pub proof fn empty_token_grants_nothing(r: u64)
    ensures
        !(has(0u64, r)),
{
    assert((0u64 & r) == 0) by (bit_vector);
}

// Functional correctness: executable functions whose bodies are the kernel's
// `src/capabilities/bits.rs` logic, token for token, and whose postconditions
// tie each to the spec function every property lemma above quantifies over.
// Verus verifies the body against the postcondition, so these establish that
// the kernel logic computes exactly the specified operation, not merely that
// the specified operation has good properties. The differential harness in
// `userland/kernel_proofs` runs the real `bits.rs` (included via #[path])
// against the same specs, so the three layers cannot drift apart silently.

/// Kernel `has_capability`: `bits & cap.bit() != 0`.
pub fn exec_has_capability(bits: u64, bit: u64) -> (r: bool)
    ensures
        r == has(bits, bit),
{
    bits & bit != 0
}

/// Kernel `add_capability`: `bits | cap.bit()`.
pub fn exec_add_capability(bits: u64, bit: u64) -> (r: u64)
    ensures
        r == grant(bits, bit),
{
    bits | bit
}

/// Kernel `remove_capability`: `bits & !cap.bit()`.
pub fn exec_remove_capability(bits: u64, bit: u64) -> (r: u64)
    ensures
        r == revoke(bits, bit),
{
    bits & !bit
}

/// Kernel attenuation: `bits & mask`.
pub fn exec_attenuate(bits: u64, mask: u64) -> (r: u64)
    ensures
        r == attenuate(bits, mask),
{
    bits & mask
}

// The lattice bounds on the machine word, matching the Lean theorems
// `Capability.attenuate_is_glb` and `Capability.grant_is_lub`. Subset order
// on tokens: `d` is contained in `t` when `d & t == d`.

/// Attenuation sits below both its operands: the two lower-bound legs.
pub proof fn attenuation_is_a_lower_bound(bits: u64, mask: u64)
    ensures
        attenuate(bits, mask) & bits == attenuate(bits, mask),
        attenuate(bits, mask) & mask == attenuate(bits, mask),
{
    assert(((bits & mask) & bits) == (bits & mask)) by (bit_vector);
    assert(((bits & mask) & mask) == (bits & mask)) by (bit_vector);
}

/// Attenuation is the greatest lower bound: any token contained in both the
/// original and the mask is contained in the attenuation. `bits & mask` loses
/// nothing both sides allow; it is the least restrictive confinement.
pub proof fn attenuation_is_the_greatest_lower_bound(bits: u64, mask: u64, d: u64)
    ensures
        (d & bits) == d && (d & mask) == d ==> (d & attenuate(bits, mask)) == d,
{
    assert((d & bits) == d && (d & mask) == d ==> (d & (bits & mask)) == d) by (bit_vector);
}

/// Granting is the least upper bound: the result contains both operands, and
/// any token containing both contains the grant. `bits | bit` adds the one
/// right and nothing else.
pub proof fn grant_is_the_least_upper_bound(bits: u64, c: u64, d: u64)
    ensures
        (bits & grant(bits, c)) == bits,
        (c & grant(bits, c)) == c,
        (bits & d) == bits && (c & d) == c ==> (grant(bits, c) & d) == grant(bits, c),
{
    assert((bits & (bits | c)) == bits) by (bit_vector);
    assert((c & (bits | c)) == c) by (bit_vector);
    assert((bits & d) == bits && (c & d) == c ==> ((bits | c) & d) == (bits | c))
        by (bit_vector);
}

/// A delegation chain collapses: attenuating by two masks in sequence equals
/// attenuating by their meet, so no chain of attenuations differs from one.
pub proof fn attenuation_chains_collapse(bits: u64, m1: u64, m2: u64)
    ensures
        attenuate(attenuate(bits, m1), m2) == attenuate(bits, m1 & m2),
{
    assert(((bits & m1) & m2) == (bits & (m1 & m2))) by (bit_vector);
}

/// Revocation is exact: it removes the named right and preserves every right
/// disjoint from it. Together with `revoke_drops_the_right` this is the full
/// functional characterization of `bits & !bit`.
pub proof fn revocation_preserves_disjoint_rights(bits: u64, c: u64, r: u64)
    ensures
        (r & c) == 0 ==> (has(revoke(bits, c), r) == has(bits, r)),
{
    assert((r & c) == 0 ==> ((((bits & !c) & r) != 0) == ((bits & r) != 0))) by (bit_vector);
}

} // verus!
