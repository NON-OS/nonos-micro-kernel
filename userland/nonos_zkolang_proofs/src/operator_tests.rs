// NONOS Operating System (AGPL-3.0-or-later)
//! The extended operators: field division, unary negation, and not-equal. Each is
//! front-end sugar over the existing opcodes, so each is proven the same way. The
//! tests check both a true program that verifies and a false claim that has none.

use nonos_zkolang::prove_source_with_inputs;

#[test]
fn division_is_multiplication_by_an_inverse() {
    // 20 / 4 == 5, exact field division.
    let report = prove_source_with_inputs("input a; input b; let q = a / b; output q;", &[20, 4])
        .expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![5]);
}

#[test]
fn division_by_zero_has_no_proof() {
    // Dividing by zero inverts zero, which has no valid trace.
    assert!(
        prove_source_with_inputs("input a; let q = a / 0; output q;", &[7]).is_err(),
        "a division by zero produced a proof"
    );
}

#[test]
fn negation_cancels_addition() {
    // (-x) + x == 0 for any x, so the assertion holds.
    let report = prove_source_with_inputs("input x; assert (-x) + x;", &[9]).expect("run");
    assert!(report.verified, "negation did not cancel");
}

#[test]
fn negation_agrees_with_subtraction_from() {
    // -x + 3 equals 3 - x.
    let report =
        prove_source_with_inputs("input x; assert (-x + 3) - (3 - x);", &[4]).expect("run");
    assert!(report.verified);
}

#[test]
fn not_equal_is_the_complement_of_equal() {
    // 3 != 5 is one.
    let differ = prove_source_with_inputs("input a; input b; let r = a != b; output r;", &[3, 5])
        .expect("run");
    assert_eq!(differ.outputs, vec![1]);
    // 3 != 3 is zero.
    let same = prove_source_with_inputs("input a; input b; let r = a != b; output r;", &[3, 3])
        .expect("run");
    assert_eq!(same.outputs, vec![0]);
}
