// NONOS Operating System (AGPL-3.0-or-later)
//! The constant-product curve. A pool that can be drained, or whose invariant
//! falls, or that rewards size, is not a pool: these are the properties a
//! trader is entitled to before any figure reaches the screen.

use crate::wallet::num::mul_div;
use crate::wallet::num::mul_wide::mul_wide;
use crate::wallet::swap::quote::{amount_out, BPS};

/// A pool with a hundred of one side and two hundred of the other, at the
/// thirty basis point fee that is the common case.
const R_IN: u128 = 100_000_000_000_000_000_000;
const R_OUT: u128 = 200_000_000_000_000_000_000;
const FEE: u32 = 30;

// The pool always keeps something back: no input, however large, empties it.
#[test]
fn a_trade_can_never_drain_the_pool() {
    for amount in [1u128, R_IN, R_IN * 1_000, u128::MAX / BPS] {
        if let Some(out) = amount_out(amount, R_IN, R_OUT, FEE) {
            assert!(out < R_OUT, "{amount} took {out} of a {R_OUT} reserve");
        }
    }
}

// The product of the reserves after the trade is at least what it was. This
// is the invariant the pool exists to hold, and the fee is what makes the
// inequality strict.
#[test]
fn the_invariant_never_falls() {
    for amount in [1_000_000u128, R_IN / 100, R_IN, R_IN * 3] {
        let out = amount_out(amount, R_IN, R_OUT, FEE).expect("quotable");
        // The product of two reserves does not fit in 128 bits, so the
        // comparison is made on the full 256-bit pair, high half first.
        let before = mul_wide(R_IN, R_OUT);
        let after = mul_wide(R_IN + amount, R_OUT - out);
        assert!(after >= before, "invariant fell on {amount}: {after:?} < {before:?}");
    }
}

// Size costs. Twice the input never returns twice the output, because the
// price moves as the trade eats into the reserve.
#[test]
fn a_larger_trade_never_gets_a_better_rate() {
    let mut previous_rate = u128::MAX;
    for amount in [R_IN / 1000, R_IN / 100, R_IN / 10, R_IN, R_IN * 2] {
        let out = amount_out(amount, R_IN, R_OUT, FEE).expect("quotable");
        let rate = mul_div(out, BPS, amount).expect("rate fits");
        assert!(rate <= previous_rate, "rate rose from {previous_rate} to {rate} at {amount}");
        previous_rate = rate;
    }
}
