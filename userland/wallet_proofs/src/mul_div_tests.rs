// NONOS Operating System (AGPL-3.0-or-later)
//! The widening multiply-divide. Every swap figure passes through it, so a
//! wrong answer here is a wrong price, and a saturated one is a price that
//! looks real.

use crate::wallet::num::mul_div;
use crate::wallet::num::mul_wide::mul_wide;

const MAX: u128 = u128::MAX;

#[test]
fn it_agrees_with_ordinary_arithmetic_where_that_fits() {
    for (a, b, d) in [(7u128, 6u128, 3u128), (1, 1, 1), (1_000_000, 999, 1_000), (5, 3, 7)] {
        assert_eq!(mul_div(a, b, d), Some(a * b / d), "{a} * {b} / {d}");
    }
}

// The case ordinary u128 cannot do: a product past 128 bits that divides
// back down into range.
#[test]
fn it_carries_a_product_that_overflows_128_bits() {
    // (2^128 - 1)^2 needs 256 bits to hold and divides back to exactly MAX.
    assert_eq!(mul_div(MAX, MAX, MAX), Some(MAX));
    // MAX * 4 passes 128 bits on the way; MAX * 4 / 8 is MAX / 2.
    assert_eq!(mul_div(MAX, 4, 8), Some(MAX / 2));
    // 2^254 / 2^126 is 2^128, one past what a u128 can hold.
    assert_eq!(mul_div(1 << 127, 1 << 127, 1 << 126), None);
}

// Truncation is toward zero, matching what the chain computes.
#[test]
fn it_truncates_rather_than_rounds() {
    assert_eq!(mul_div(10, 10, 3), Some(33));
    assert_eq!(mul_div(1, 1, 2), Some(0));
    assert_eq!(mul_div(MAX, 1, MAX), Some(1));
}

// A refusal, never a saturation: both are cases where no 128-bit answer
// exists, and returning a number would be inventing one.
#[test]
fn it_refuses_where_there_is_no_answer() {
    assert_eq!(mul_div(1, 1, 0), None, "divide by zero");
    assert_eq!(mul_div(MAX, MAX, 1), None, "quotient past 128 bits");
    assert_eq!(mul_div(MAX, 2, 1), None);
}

#[test]
fn zero_is_zero_from_either_side() {
    assert_eq!(mul_div(0, MAX, 7), Some(0));
    assert_eq!(mul_div(MAX, 0, 7), Some(0));
}

// The 256-bit product itself, checked against values whose halves are known.
#[test]
fn the_wide_product_splits_where_it_should() {
    assert_eq!(mul_wide(0, 0), (0, 0));
    assert_eq!(mul_wide(1, 1), (0, 1));
    assert_eq!(mul_wide(1 << 64, 1 << 64), (1, 0));
    assert_eq!(mul_wide(MAX, MAX), (MAX - 1, 1));
    assert_eq!(mul_wide(MAX, 2), (1, MAX - 1));
}

// Exhaustive over a small space: every triple must match the same figure
// computed the wide way by hand.
#[test]
fn it_matches_a_reference_over_a_small_exhaustive_space() {
    for a in 0u128..40 {
        for b in 0u128..40 {
            for d in 1u128..40 {
                assert_eq!(mul_div(a, b, d), Some(a * b / d), "{a} {b} {d}");
            }
        }
    }
}
