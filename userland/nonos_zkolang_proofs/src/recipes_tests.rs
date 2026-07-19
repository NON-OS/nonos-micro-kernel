// NONOS Operating System (AGPL-3.0-or-later)
//! Recipes: real verifiable-compute use cases expressed in zKølang and proven end
//! to end. Each is a use a buyer would actually pay to have proven. They double as
//! the tested source behind the recipes documentation page.

use nonos_zkolang::{prove_source_with_inputs, prove_source_with_witness};

// A delegated public computation: prove y = 3x^2 + 2x + 5 for a public x, so a
// verifier trusts the output without running the polynomial.
#[test]
fn polynomial_evaluation() {
    let src = "input x; let y = 3 * x * x + 2 * x + 5; output y;";
    let report = prove_source_with_inputs(src, &[2]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![21], "3*4 + 4 + 5 = 21");
}

// Knowledge of a secret root of a public quadratic: prove there is an x with
// x^2 + c == b*x, without revealing x. Roots of x^2 - 5x + 6 are 2 and 3.
#[test]
fn secret_root_of_a_public_quadratic() {
    let src = "input b; input c; secret x; assert x * x + c - b * x;";
    // b = 5, c = 6, x = 2: 4 + 6 - 10 = 0.
    assert!(prove_source_with_witness(src, &[5, 6], &[2]).expect("run").verified);
    // The other root, x = 3: 9 + 6 - 15 = 0.
    assert!(prove_source_with_witness(src, &[5, 6], &[3]).expect("run").verified);
    // A non-root, x = 4: 16 + 6 - 20 = 2, no proof.
    assert!(prove_source_with_witness(src, &[5, 6], &[4]).is_err());
}

// Private set membership: prove a secret w is one of a public allowlist
// {v0, v1, v2}, without revealing which. The product is zero exactly when w
// equals one of them.
#[test]
fn private_set_membership() {
    let src = "secret w; input v0; input v1; input v2;
               assert (w - v0) * (w - v1) * (w - v2);";
    // w = 7 is in {5, 7, 9}.
    assert!(prove_source_with_witness(src, &[5, 7, 9], &[7]).expect("run").verified);
    // w = 8 is not in the set, so there is no proof.
    assert!(prove_source_with_witness(src, &[5, 7, 9], &[8]).is_err());
}

// Solvency by conservation: prove knowledge of three private balances that sum to
// a public total, without revealing the balances.
#[test]
fn private_balances_sum_to_a_public_total() {
    let src = "secret a; secret b; secret c; let s = a + b + c; output s;";
    let report = prove_source_with_witness(src, &[], &[10, 20, 30]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![60], "10 + 20 + 30 = 60");
}

// A range proof: prove a secret w lies in [0, 16) by exhibiting its four bits,
// each constrained boolean, that reconstruct it. This is the pattern behind
// proving an age or a balance is in range without revealing it. It needs register
// reuse to fit the file.
#[test]
fn secret_value_is_in_range() {
    let src = "secret w;
               secret b0; secret b1; secret b2; secret b3;
               assert b0 * b0 - b0;
               assert b1 * b1 - b1;
               assert b2 * b2 - b2;
               assert b3 * b3 - b3;
               assert w - (b0 + 2 * b1 + 4 * b2 + 8 * b3);";
    // w = 13 = 1101: bits b0=1, b1=0, b2=1, b3=1.
    assert!(prove_source_with_witness(src, &[], &[13, 1, 0, 1, 1]).expect("run").verified);
    // A non-boolean bit, b0 = 2, is not a valid decomposition, so no proof.
    assert!(prove_source_with_witness(src, &[], &[13, 2, 0, 1, 1]).is_err());
}

// The same range proof written with a function. `bit(b)` is the boolean relation
// once, and each bit asserts it, so the repeated `b * b - b` becomes one named
// thing. The proof is identical, because the call is inlined; functions cut the
// source, not the trace.
#[test]
fn a_range_proof_with_a_reusable_bit_relation() {
    let src = "fn bit(b) = b * b - b;
               secret w;
               secret b0; secret b1; secret b2; secret b3;
               assert bit(b0);
               assert bit(b1);
               assert bit(b2);
               assert bit(b3);
               assert w - (b0 + 2 * b1 + 4 * b2 + 8 * b3);";
    // w = 10 = 1010: b0=0, b1=1, b2=0, b3=1.
    assert!(prove_source_with_witness(src, &[], &[10, 0, 1, 0, 1]).expect("run").verified);
    // A bit that is not boolean fails the `bit` relation, so there is no proof.
    assert!(prove_source_with_witness(src, &[], &[10, 3, 1, 0, 1]).is_err());
}

// Delegated computation with a function library: prove a weighted sum of squares
// for public inputs, with the square and the multiply-add each named once. The
// verifier trusts the output without running the arithmetic.
#[test]
fn weighted_sum_of_squares_with_functions() {
    let src = "fn sq(x) = x * x;
               fn madd(a, b, c) = a * b + c;
               input x; input y;
               let r = madd(sq(x), 3, sq(y));   // 3*x^2 + y^2
               output r;";
    let report = prove_source_with_inputs(src, &[2, 5]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![37], "3*4 + 25 = 37");
}
