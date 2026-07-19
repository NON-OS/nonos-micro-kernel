// NONOS Operating System (AGPL-3.0-or-later)
//! The conditional expression, the comparison-aware assert, and the register
//! reclaim on shadowing. Each is front-end ergonomics over the proven core, so
//! each proves the same way. The long accumulator loop is the one that would have
//! exhausted the register file before shadowed bindings were reclaimed.

use nonos_zkolang::prove_source_with_inputs;

#[test]
fn if_expression_selects_a_branch() {
    // if c { 10 } else { 20 } is sel(c, 10, 20).
    let src = "input c; let x = if c { 10 } else { 20 }; output x;";
    assert_eq!(prove_source_with_inputs(src, &[1]).expect("run").outputs, vec![10]);
    assert_eq!(prove_source_with_inputs(src, &[0]).expect("run").outputs, vec![20]);
}

#[test]
fn if_expression_over_a_computed_condition() {
    // Nonzero maps to branch a, zero to branch b, via a != 0 as the bit.
    let src = "input a; let flag = if a != 0 { 100 } else { 200 }; output flag;";
    assert_eq!(prove_source_with_inputs(src, &[7]).expect("run").outputs, vec![100]);
    assert_eq!(prove_source_with_inputs(src, &[0]).expect("run").outputs, vec![200]);
}

#[test]
fn assert_equal_reads_naturally() {
    // assert a == b proves equality; a mismatch has no proof.
    let src = "input a; input b; assert a == b;";
    assert!(prove_source_with_inputs(src, &[5, 5]).expect("run").verified);
    assert!(prove_source_with_inputs(src, &[5, 6]).is_err());
}

#[test]
fn assert_not_equal_reads_naturally() {
    // assert a != b proves inequality; equal values have no proof.
    let src = "input a; input b; assert a != b;";
    assert!(prove_source_with_inputs(src, &[5, 6]).expect("run").verified);
    assert!(prove_source_with_inputs(src, &[5, 5]).is_err());
}

#[test]
fn shadowing_reclaims_a_register_but_keeps_aliases() {
    // b aliases a's register; rebinding a must not disturb b.
    let src = "let a = 3; let b = a; let a = 5; output b;";
    let report = prove_source_with_inputs(src, &[]).expect("run");
    assert_eq!(report.outputs, vec![3], "an alias was clobbered by a shadow");
}

#[test]
fn a_long_accumulator_loop_fits_after_reclaim() {
    // Thirty iterations of a shadowing accumulator. Without reclaiming the old
    // binding's register each iteration, this overflows the sixteen-register file;
    // with it, peak pressure is a handful of registers. Sum of 0..30 is 435.
    let src = "let acc = 0; for i in 0 .. 30 { let acc = acc + i; } output acc;";
    let report = prove_source_with_inputs(src, &[]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![435]);
}
