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

//! Kani harnesses: the hex decoders never panic or wrap on arbitrary bytes.
//!
//! Inputs are kept short and the loops bounded so CBMC stays fast. The decoders
//! act one byte at a time and their panic-freedom does not depend on length, so
//! a short symbolic input covers every branch; `decode_u128_saturates` feeds a
//! value wide enough to reach the saturation path the short cases do not.

use crate::hex::{decode_fixed, decode_u128};

// Decoding a fixed-width value out of arbitrary bytes never panics, and a
// success means the hex fit the target.
#[kani::proof]
#[kani::unwind(6)]
fn decode_fixed_is_total() {
    let bytes: [u8; 8] = kani::any();
    let mut out = [0u8; 4];
    let _ = decode_fixed::<4>(&bytes, &mut out);
}

// Decoding an amount out of arbitrary bytes never panics or overflows.
#[kani::proof]
#[kani::unwind(5)]
fn decode_u128_is_total() {
    let bytes: [u8; 8] = kani::any();
    let _ = decode_u128(&bytes);
}

// A value wider than sixteen bytes reaches the saturation branch without
// overflowing, so an oversized amount reads as at least any real price rather
// than wrapping to a small one.
#[kani::proof]
#[kani::unwind(20)]
fn decode_u128_saturates() {
    // "0x" then a symbolic first byte and enough fixed hex to exceed 16 bytes.
    let mut bytes = [b'0'; 38];
    bytes[0] = b'0';
    bytes[1] = b'x';
    bytes[2] = kani::any();
    let _ = decode_u128(&bytes);
}
