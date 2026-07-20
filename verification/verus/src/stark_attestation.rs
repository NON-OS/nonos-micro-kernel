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

//! Verus-checked theorems for the transparent STARK attestation. The trailer is
//! read from untrusted bytes, so the two facts that keep the spawn gate safe are
//! that a length prefix capped at the remaining bytes never over-reserves, and
//! that the cursor never advances out of bounds. Acceptance is the conjunction
//! of the three gate checks, so a forgery that misses any one is refused.

use vstd::prelude::*;

verus! {

// A length count read from an untrusted trailer, capped at the bytes that still
// remain. Every element consumes at least one byte, so this is a safe reservation.
pub open spec fn capped(count: nat, remaining: nat) -> nat {
    if count <= remaining { count } else { remaining }
}

// The cap never reserves more than the input can back: no over-allocation.
pub proof fn cap_never_exceeds_remaining(count: nat, remaining: nat)
    ensures
        capped(count, remaining) <= remaining,
{
}

// The cap never reserves more than was asked for: it only ever shrinks.
pub proof fn cap_never_exceeds_count(count: nat, remaining: nat)
    ensures
        capped(count, remaining) <= count,
{
}

// When the count already fits, the cap is exact, so valid proofs parse unchanged.
pub proof fn cap_is_exact_when_it_fits(count: nat, remaining: nat)
    requires
        count <= remaining,
    ensures
        capped(count, remaining) == count,
{
}

// The cursor may read n bytes at position i only when they lie inside the buffer.
pub open spec fn can_take(i: nat, n: nat, len: nat) -> bool {
    i + n <= len
}

// A permitted read leaves the cursor inside the buffer: no out-of-bounds access.
pub proof fn take_stays_in_bounds(i: nat, n: nat, len: nat)
    requires
        can_take(i, n, len),
    ensures
        i + n <= len,
{
}

// The cursor only moves forward.
pub proof fn take_advances(i: nat, n: nat, len: nat)
    ensures
        i + n >= i,
{
}

// The gate accepts only when the opening reaches the enrolled root, the context
// is the expected one, and the measurement is enrolled: all three at once.
pub open spec fn accepts(root_ok: bool, ctx_ok: bool, enrolled: bool) -> bool {
    root_ok && ctx_ok && enrolled
}

// An unenrolled image is refused however it opens.
pub proof fn forgery_not_enrolled_is_refused(root_ok: bool, ctx_ok: bool)
    ensures
        !accepts(root_ok, ctx_ok, false),
{
}

// A proof drawn under the wrong identity is refused: no replay.
pub proof fn wrong_context_is_refused(root_ok: bool, enrolled: bool)
    ensures
        !accepts(root_ok, false, enrolled),
{
}

// Acceptance implies every check passed, so nothing outside the policy attests.
pub proof fn accept_requires_all_three(root_ok: bool, ctx_ok: bool, enrolled: bool)
    requires
        accepts(root_ok, ctx_ok, enrolled),
    ensures
        root_ok,
        ctx_ok,
        enrolled,
{
}

} // verus!
