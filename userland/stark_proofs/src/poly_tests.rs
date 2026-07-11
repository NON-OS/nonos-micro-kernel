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

use crate::crypto::stark::field::{Fp, Fp2};
use crate::crypto::stark::poly::{eval, eval_ext, eval_lagrange};

extern crate alloc;
use alloc::vec::Vec;

// Polynomials are the algebraic core of a STARK: a computation is encoded as a
// low-degree polynomial, and the proof convinces a verifier the polynomial is
// indeed low degree. These proofs check evaluation and the low-degree extension
// against their defining properties over adversarial coefficients and points.

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

fn coeffs(n: usize, state: &mut u64) -> Vec<Fp> {
    (0..n).map(|_| Fp::from_u64(xorshift(state))).collect()
}

#[test]
fn evaluation_matches_the_defining_sum() {
    let mut s = 0x1234_5678_9abc_def0u64;
    for _ in 0..200_000 {
        let x = Fp::from_u64(xorshift(&mut s));
        // Constant and linear cases pin the endpoints of Horner's method.
        let c = Fp::from_u64(xorshift(&mut s));
        assert_eq!(eval(&[c], x), c, "a constant evaluates to itself");
        let b = Fp::from_u64(xorshift(&mut s));
        assert_eq!(eval(&[c, b], x), c + b * x, "a + b x");
        // Evaluation is additive in the coefficients.
        let p = coeffs(6, &mut s);
        let q = coeffs(6, &mut s);
        let sum: Vec<Fp> = p.iter().zip(&q).map(|(&a, &b)| a + b).collect();
        assert_eq!(eval(&sum, x), eval(&p, x) + eval(&q, x), "eval is additive");
    }
}

#[test]
fn lagrange_interpolation_reproduces_its_nodes() {
    let mut s = 0xdead_beef_cafe_babeu64;
    for _ in 0..20_000 {
        let n = 2 + (xorshift(&mut s) % 12) as usize;
        // Distinct evaluation points, as an interpolation requires.
        let xs: Vec<Fp> = (0..n).map(|i| Fp::from_u64(i as u64 + 1)).collect();
        let ys = coeffs(n, &mut s);
        for k in 0..n {
            assert_eq!(eval_lagrange(&xs, &ys, xs[k]), ys[k], "interpolant misses node {k}");
        }
    }
}

#[test]
fn the_low_degree_extension_recovers_the_polynomial() {
    // The heart of it: sampling a degree < n polynomial at n distinct points and
    // interpolating recovers the same polynomial everywhere. This is what makes
    // a trace and its low-degree extension interchangeable.
    let mut s = 0x0f1e_2d3c_4b5a_6978u64;
    for _ in 0..20_000 {
        let n = 2 + (xorshift(&mut s) % 12) as usize;
        let p = coeffs(n, &mut s);
        let xs: Vec<Fp> = (0..n).map(|i| Fp::from_u64(i as u64 + 1)).collect();
        let ys: Vec<Fp> = xs.iter().map(|&x| eval(&p, x)).collect();
        for _ in 0..8 {
            let z = Fp::from_u64(xorshift(&mut s));
            assert_eq!(
                eval_lagrange(&xs, &ys, z),
                eval(&p, z),
                "the interpolant disagrees with the polynomial off the nodes"
            );
        }
    }
}

// The out-of-domain sampling point of a money-grade STARK is drawn from Fp2, so a
// base-coefficient trace column is evaluated there via eval_ext. It must agree
// with eval on base points and be a genuine Fp2 evaluation off them.

#[test]
fn eval_ext_agrees_with_eval_on_base_points() {
    let mut s = 0x9e37u64 | 1;
    let coeffs: Vec<Fp> = (0..24).map(|_| Fp::from_u64(xorshift(&mut s))).collect();
    for _ in 0..500 {
        let x = Fp::from_u64(xorshift(&mut s));
        assert_eq!(eval_ext(&coeffs, Fp2::from_base(x)), Fp2::from_base(eval(&coeffs, x)));
    }
}

#[test]
fn eval_ext_matches_the_horner_definition_in_the_extension() {
    let mut s = 0xC0DEu64 | 1;
    let coeffs: Vec<Fp> = (0..8).map(|_| Fp::from_u64(xorshift(&mut s))).collect();
    for _ in 0..200 {
        let x = Fp2::new(Fp::from_u64(xorshift(&mut s)), Fp::from_u64(xorshift(&mut s)));
        // Horner reference computed independently.
        let mut acc = Fp2::ZERO;
        for &c in coeffs.iter().rev() {
            acc = acc * x + Fp2::from_base(c);
        }
        assert_eq!(eval_ext(&coeffs, x), acc);
    }
}
