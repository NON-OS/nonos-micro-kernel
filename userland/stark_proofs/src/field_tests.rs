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

use crate::crypto::stark::field::{Fp, P};

// The Goldilocks field is the base of every STARK. If its arithmetic is wrong,
// nothing above it can be sound. These proofs check the real implementation
// against the field axioms over a large adversarial sample, which catches any
// carry or reduction defect immediately: an inverse or Fermat check fails the
// instant a product is off by anything.

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

fn elem(state: &mut u64) -> Fp {
    Fp::from_u64(xorshift(state))
}

#[test]
fn canonical_representatives_and_edge_values() {
    assert_eq!(Fp::from_u64(0), Fp::ZERO);
    assert_eq!(Fp::from_u64(P).value(), 0);
    assert_eq!(Fp::from_u64(P - 1).value(), P - 1);
    assert_eq!(Fp::from_u64(u64::MAX).value(), u64::MAX - P);
    // (p - 1) + 1 = 0 in the field.
    assert_eq!(Fp::from_u64(P - 1) + Fp::ONE, Fp::ZERO);
    // 0 - 1 = p - 1.
    assert_eq!((Fp::ZERO - Fp::ONE).value(), P - 1);
}

#[test]
fn addition_is_an_abelian_group() {
    let mut s = 0x1234_5678_9abc_def0;
    for _ in 0..500_000 {
        let (a, b, c) = (elem(&mut s), elem(&mut s), elem(&mut s));
        assert_eq!(a + b, b + a, "add commutes");
        assert_eq!((a + b) + c, a + (b + c), "add associates");
        assert_eq!(a + Fp::ZERO, a, "zero is the identity");
        assert_eq!(a + (-a), Fp::ZERO, "negation inverts");
        assert_eq!((a - b) + b, a, "subtraction undoes addition");
        assert!(a.value() < P, "every representative is canonical");
    }
}

#[test]
fn multiplication_is_a_commutative_monoid_and_distributes() {
    let mut s = 0xdead_beef_cafe_babe;
    for _ in 0..500_000 {
        let (a, b, c) = (elem(&mut s), elem(&mut s), elem(&mut s));
        assert_eq!(a * b, b * a, "mul commutes");
        assert_eq!((a * b) * c, a * (b * c), "mul associates");
        assert_eq!(a * Fp::ONE, a, "one is the identity");
        assert_eq!(a * Fp::ZERO, Fp::ZERO, "zero absorbs");
        // Distributivity ties the two operations together and is the strongest
        // single check on the reduction.
        assert_eq!(a * (b + c), (a * b) + (a * c), "mul distributes over add");
        assert_eq!(a.square(), a * a, "square is self multiplication");
    }
}

#[test]
fn every_nonzero_element_has_an_inverse() {
    let mut s = 0x0f1e_2d3c_4b5a_6978;
    for _ in 0..100_000 {
        let a = elem(&mut s);
        if a == Fp::ZERO {
            continue;
        }
        assert_eq!(a * a.inv(), Fp::ONE, "a * a^-1 = 1");
        // Fermat's little theorem: a^(p-1) = 1 for every nonzero a.
        assert_eq!(a.pow(P - 1), Fp::ONE, "a^(p-1) = 1");
    }
}

#[test]
fn exponentiation_agrees_with_repeated_multiplication() {
    let mut s = 0xa5a5_5a5a_c3c3_3c3c;
    for _ in 0..50_000 {
        let a = elem(&mut s);
        let mut acc = Fp::ONE;
        for k in 0u64..12 {
            assert_eq!(a.pow(k), acc, "a^k matches the running product");
            acc = acc * a;
        }
    }
}
