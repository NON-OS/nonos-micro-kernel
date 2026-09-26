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

//! The scalar field the Pedersen and Schnorr proofs use, against values worked
//! out with exact integers in Python (a * b mod L, a + b mod L, a - b mod L).

use crate::crypto::zk_kernel::FieldElement;
use crate::zk_field_vectors::{ARITH, WIDE};

fn b32(s: &str) -> [u8; 32] {
    let mut out = [0u8; 32];
    for (i, o) in out.iter_mut().enumerate() {
        *o = u8::from_str_radix(&s[2 * i..2 * i + 2], 16).unwrap();
    }
    out
}

fn b64(s: &str) -> [u8; 64] {
    let mut out = [0u8; 64];
    for (i, o) in out.iter_mut().enumerate() {
        *o = u8::from_str_radix(&s[2 * i..2 * i + 2], 16).unwrap();
    }
    out
}

#[test]
fn field_arithmetic_is_arithmetic_mod_l() {
    for (i, (a, b, mul, add, sub)) in ARITH.iter().enumerate() {
        let (x, y) = (FieldElement::from_bytes(&b32(a)), FieldElement::from_bytes(&b32(b)));
        assert_eq!(x.mul(&y).to_bytes(), b32(mul), "mul, case {i}");
        assert_eq!(x.add(&y).to_bytes(), b32(add), "add, case {i}");
        assert_eq!(x.sub(&y).to_bytes(), b32(sub), "sub, case {i}");
    }
}

#[test]
fn a_wide_value_reduces_mod_l() {
    for (i, (x, want)) in WIDE.iter().enumerate() {
        assert_eq!(FieldElement::from_bytes_wide(&b64(x)).to_bytes(), b32(want), "case {i}");
    }
}
