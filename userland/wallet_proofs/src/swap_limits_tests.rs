// NONOS Operating System (AGPL-3.0-or-later)
//! What the reading refuses, and the two costs it keeps apart. Impact is
//! the curve; the fee is quoted on its own line. A screen that folded them
//! together would make a cheap pool look like a thin one.

use crate::wallet::swap::quote::{amount_out, impact_bps};

/// A pool with a hundred of one side and two hundred of the other, at the
/// thirty basis point fee that is the common case.
const R_IN: u128 = 100_000_000_000_000_000_000;
const R_OUT: u128 = 200_000_000_000_000_000_000;
const FEE: u32 = 30;

// Impact grows with size, and a trade small against the reserve barely
// registers. A screen that showed a flat impact would hide the cost of size.
#[test]
fn impact_grows_with_the_size_of_the_trade() {
    let small = R_IN / 10_000;
    let large = R_IN;
    let out_small = amount_out(small, R_IN, R_OUT, FEE).expect("quotable");
    let out_large = amount_out(large, R_IN, R_OUT, FEE).expect("quotable");
    let i_small = impact_bps(small, R_IN, R_OUT, out_small).expect("measurable");
    let i_large = impact_bps(large, R_IN, R_OUT, out_large).expect("measurable");
    assert!(i_small < i_large, "impact did not grow: {i_small} then {i_large}");
    assert!(i_small < 100, "a trade of a ten thousandth should be under one percent");
    assert!(i_large > 1_000, "a trade the size of the reserve should read as dangerous");
}

// Impact is the curve, not the fee. A vanishing trade pays the fee and still
// reports no meaningful impact, so the two costs stay separable on screen.
#[test]
fn impact_does_not_double_count_the_fee() {
    let tiny = R_IN / 1_000_000;
    let out = amount_out(tiny, R_IN, R_OUT, FEE).expect("quotable");
    let impact = impact_bps(tiny, R_IN, R_OUT, out).expect("measurable");
    assert!(impact <= FEE + 1, "impact {impact} is carrying the {FEE} bps fee");
}

// Every unquotable case is a refusal. Zero is a real answer to a real trade,
// so it must never stand in for "no answer".
#[test]
fn it_refuses_rather_than_quoting_zero() {
    assert_eq!(amount_out(0, R_IN, R_OUT, FEE), None, "no input");
    assert_eq!(amount_out(1, 0, R_OUT, FEE), None, "empty input side");
    assert_eq!(amount_out(1, R_IN, 0, FEE), None, "empty output side");
    assert_eq!(amount_out(1, R_IN, R_OUT, 10_000), None, "fee takes everything");
    assert_eq!(amount_out(u128::MAX, R_IN, R_OUT, FEE), None, "input past what multiplies");
    assert_eq!(impact_bps(1, 0, R_OUT, 0), None, "no mid price to compare against");
}

// A zero fee is a real configuration and must quote, not refuse.
#[test]
fn a_zero_fee_pool_still_quotes() {
    let out = amount_out(R_IN / 100, R_IN, R_OUT, 0).expect("quotable");
    let with_fee = amount_out(R_IN / 100, R_IN, R_OUT, FEE).expect("quotable");
    assert!(out > with_fee, "a free pool should return more than a charging one");
}
