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

//! Forcing s into the lower half of the group order.
//!
//! For any valid signature, `(r, s)` and `(r, n - s)` are both valid over the
//! same message and key. Left alone that is a malleability: the same
//! transaction can be resubmitted under a second hash. Ethereum and Bitcoin
//! both reject the high form outright, so signing picks the low one.
//!
//! Flipping s flips the parity of R that a verifier will recover, so the
//! recovery id has to flip with it. That coupling is why the two live here
//! together rather than the caller doing half of it.

use crate::scalar::Scalar;

/*
 * (n - 1) / 2, little-endian limbs. An s at or below this is already the low
 * one of the pair.
 */
const HALF_N: [u64; 4] =
    [0xDFE92F46681B20A0, 0x5D576E7357A4501D, 0xFFFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF];

/// 1 when `s > HALF_N`, without branching on a value derived from the key.
///
/// The limbs are walked most significant first. `undecided` stays 1 until a
/// limb pair differs and then latches to 0, so every later limb is read but
/// cannot change the answer. Both comparisons come from the borrow of a
/// wrapping subtraction rather than from `<`, which on some targets compiles
/// to a branch.
fn is_high(s: &Scalar) -> u64 {
    let mut high = 0u64;
    let mut undecided = 1u64;
    for i in (0..4).rev() {
        let a = s.0[i];
        let b = HALF_N[i];
        let lt = (a ^ ((a ^ b) | (a.wrapping_sub(b) ^ b))) >> 63;
        let gt = (b ^ ((b ^ a) | (b.wrapping_sub(a) ^ a))) >> 63;
        high |= undecided & gt;
        undecided &= 1 ^ (gt | lt);
    }
    high
}

/// The low form of s, and the recovery id it goes with.
///
/// `y_parity` is the parity of R's y coordinate as signing computed it. A
/// verifier recovering from the negated s sees the opposite parity, so the id
/// is that parity XOR whether the flip happened.
pub(crate) fn normalise(s: Scalar, y_parity: u8) -> (Scalar, u8) {
    let high = is_high(&s);
    let negated = s.negate();
    let mask = 0u64.wrapping_sub(high);
    (Scalar::ct_select(mask, &negated, &s), y_parity ^ (high as u8))
}
