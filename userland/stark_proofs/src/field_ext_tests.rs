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

use crate::crypto::stark::field::{Fp, Fp2, P};

// The degree-2 extension Fp2 = Fp[X]/(X^2 - 7) is where FRI and DEEP challenges
// are drawn, so the low-degree test reaches ~2^-128 soundness instead of the
// ~2^-64 the base field caps it at. If its arithmetic is wrong the whole
// soundness argument is void, so these check the real implementation against the
// field axioms over a large adversarial sample, exactly as the base field is.

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

fn elem(state: &mut u64) -> Fp {
    Fp::from_u64(xorshift(state))
}

fn ext_elem(state: &mut u64) -> Fp2 {
    Fp2::new(elem(state), elem(state))
}

#[test]
fn seven_is_a_quadratic_non_residue() {
    // X^2 - 7 is irreducible over Goldilocks iff 7 is a non-residue, which holds
    // iff 7^((p-1)/2) = -1. This is what makes the quotient a field at all.
    let legendre = Fp::from_u64(7).pow((P - 1) / 2);
    assert_eq!(legendre, Fp::from_u64(P - 1));
}

#[test]
fn multiplication_matches_the_polynomial_definition() {
    let mut s = 0xabcd_1234_5678_9f01;
    let w = Fp::from_u64(7);
    for _ in 0..4000 {
        let a = ext_elem(&mut s);
        let b = ext_elem(&mut s);
        // (a0 + a1 X)(b0 + b1 X) = (a0 b0 + 7 a1 b1) + (a0 b1 + a1 b0) X.
        let c0 = a.c0 * b.c0 + w * (a.c1 * b.c1);
        let c1 = a.c0 * b.c1 + a.c1 * b.c0;
        assert_eq!(a * b, Fp2::new(c0, c1));
    }
}

#[test]
fn multiplication_is_commutative_and_distributes() {
    let mut s = 0x1111_2222_3333_4444;
    for _ in 0..4000 {
        let a = ext_elem(&mut s);
        let b = ext_elem(&mut s);
        let c = ext_elem(&mut s);
        assert_eq!(a * b, b * a);
        assert_eq!(a * (b + c), a * b + a * c);
    }
}

#[test]
fn every_nonzero_element_has_an_inverse() {
    let mut s = 0x9999_8888_7777_6666;
    for _ in 0..8000 {
        let a = ext_elem(&mut s);
        if a == Fp2::ZERO {
            continue;
        }
        // A nonzero element has a nonzero norm, since X^2 - 7 has no base root.
        assert_ne!(a.norm(), Fp::ZERO);
        assert_eq!(a * a.inv(), Fp2::ONE);
    }
}

#[test]
fn conjugate_times_element_is_the_norm() {
    let mut s = 0x0f0f_0f0f_0f0f_0f0f;
    for _ in 0..4000 {
        let a = ext_elem(&mut s);
        // a * conj(a) = N(a), a base element embedded back into Fp2.
        assert_eq!(a * a.conjugate(), Fp2::from_base(a.norm()));
    }
}

#[test]
fn the_base_embedding_is_a_ring_homomorphism() {
    let mut s = 0x5a5a_5a5a_5a5a_5a5a;
    for _ in 0..4000 {
        let a = elem(&mut s);
        let b = elem(&mut s);
        assert_eq!(Fp2::from_base(a) + Fp2::from_base(b), Fp2::from_base(a + b));
        assert_eq!(Fp2::from_base(a) * Fp2::from_base(b), Fp2::from_base(a * b));
    }
}

// The Felt abstraction lets one AIR constraint be evaluated over the base field
// (coset composition) or the extension (out-of-domain sampling). A generic
// routine written against Felt must give matching, embedded results on both.

use crate::crypto::stark::field::Felt;

/// A small constraint-shaped polynomial written once, generic over the field:
/// c(x) = x*(x-1) + inv(x+1) evaluated with the trait ops only.
fn generic_expr<F: Felt>(x: F) -> F {
    let one = F::ONE;
    x * (x - one) + (x + one).inv()
}

#[test]
fn a_generic_expression_matches_across_base_and_extension() {
    let mut s = 0x7e57u64 | 1;
    for _ in 0..500 {
        let xb = Fp::from_u64(xorshift(&mut s));
        // Evaluate over Fp, and over Fp2 at the embedded point; they must agree
        // once the base result is embedded.
        let base = generic_expr(xb);
        let ext = generic_expr(Fp2::from_base(xb));
        assert_eq!(Fp2::from_base(base), ext);
    }
}

#[test]
fn felt_pow_and_inv_are_consistent_in_both_fields() {
    let mut s = 0xF00Du64 | 1;
    for _ in 0..300 {
        let xb = Fp::from_u64(xorshift(&mut s));
        // pow embeds, and inv times value is one, in each field via the trait.
        assert_eq!(Fp2::from_base(Felt::pow(xb, 5)), Felt::pow(Fp2::from_base(xb), 5));
        let xe = Fp2::new(Fp::from_u64(xorshift(&mut s)), Fp::from_u64(xorshift(&mut s)));
        if xe != Fp2::ZERO {
            assert_eq!(xe * Felt::inv(xe), Fp2::ONE);
        }
    }
}
