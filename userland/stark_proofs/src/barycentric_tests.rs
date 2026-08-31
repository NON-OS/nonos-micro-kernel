// NONOS Operating System (AGPL-3.0-or-later)
//! The barycentric evaluation must agree with the general Lagrange one on every
//! input the verifier can hand it, because it replaces it there. The general
//! form is quadratic in the domain size and is kept as the reference these check
//! against, and as the fallback when the point lands inside the domain.

use crate::crypto::stark::field::{Fp, Fp2};
use crate::crypto::stark::fri::root_of_unity;
use crate::crypto::stark::poly::{eval_lagrange_ext, eval_subgroup_ext};
use alloc::vec::Vec;

fn domain(log_t: u32) -> (Fp, Vec<Fp>) {
    let t = 1usize << log_t;
    let g = root_of_unity(log_t);
    let mut pts = Vec::with_capacity(t);
    let mut p = Fp::ONE;
    for _ in 0..t {
        pts.push(p);
        p = p * g;
    }
    (g, pts)
}

fn values(t: usize, salt: u64) -> Vec<Fp> {
    (0..t)
        .map(|i| Fp::from_u64((i as u64).wrapping_mul(2654435761).wrapping_add(salt) | 1))
        .collect()
}

#[test]
fn it_agrees_with_the_general_lagrange_evaluation() {
    for log_t in 1u32..=8 {
        let t = 1usize << log_t;
        let (g, pts) = domain(log_t);
        let ys = values(t, log_t as u64 * 7 + 1);
        for k in 0..4u64 {
            let z = Fp2 { c0: Fp::from_u64(1234 + k * 99), c1: Fp::from_u64(7 + k) };
            let reference = eval_lagrange_ext(&pts, &ys, z);
            let fast = eval_subgroup_ext(g, &ys, z).expect("z is outside the domain");
            assert_eq!(reference, fast, "log_t {log_t}, z index {k}");
        }
    }
}

#[test]
fn it_reproduces_the_values_it_interpolates() {
    // A point just off the domain still has to carry the polynomial the values
    // define, so check against the reference at a shifted domain point.
    let log_t = 6u32;
    let t = 1usize << log_t;
    let (g, pts) = domain(log_t);
    let ys = values(t, 31);
    for i in [0usize, 1, t / 2, t - 1] {
        let z = Fp2 { c0: pts[i], c1: Fp::ONE };
        assert_eq!(
            eval_lagrange_ext(&pts, &ys, z),
            eval_subgroup_ext(g, &ys, z).expect("off the base-field domain"),
            "shifted point {i}"
        );
    }
}

#[test]
fn a_point_inside_the_domain_has_no_barycentric_form() {
    let log_t = 4u32;
    let (g, pts) = domain(log_t);
    let ys = values(1 << log_t, 5);
    for i in [0usize, 3, 15] {
        let z = Fp2::from_base(pts[i]);
        assert!(eval_subgroup_ext(g, &ys, z).is_none(), "domain point {i} should fall back");
    }
}

#[test]
fn the_zero_polynomial_evaluates_to_zero() {
    let log_t = 5u32;
    let (g, _) = domain(log_t);
    let ys = alloc::vec![Fp::ZERO; 1 << log_t];
    let z = Fp2 { c0: Fp::from_u64(9182), c1: Fp::from_u64(3) };
    assert_eq!(eval_subgroup_ext(g, &ys, z), Some(Fp2::ZERO));
}
