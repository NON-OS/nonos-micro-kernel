// NONOS Operating System (AGPL-3.0-or-later)
//! Bounded `for` loops, unrolled by the compiler. The loop variable is a
//! compile-time constant, so a program's shape stays static; a loop is exactly
//! its hand-unrolled body, and it proves the same way. These cover an
//! accumulator, a power by repeated multiply, inputs inside a loop, an empty
//! range, nesting, and the too-large guard.

use nonos_zkolang::{compile_source, prove_source_with_inputs, CompileError};

#[test]
fn loop_accumulates_over_its_variable() {
    // acc = 0 + 0 + 1 + 2 + 3 = 6.
    let src = "let acc = 0; for i in 0 .. 4 { let acc = acc + i; } output acc;";
    let report = prove_source_with_inputs(src, &[]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![6]);
}

#[test]
fn loop_computes_a_power() {
    // p = x^3 by multiplying three times.
    let src = "input x; let p = 1; for i in 0 .. 3 { let p = p * x; } output p;";
    let report = prove_source_with_inputs(src, &[2]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![8]);
}

#[test]
fn loop_sums_inputs_it_reads() {
    // Three public inputs are read inside the loop and summed. The input count is
    // computed through the loop, so the public prefix is sized right.
    let src = "let acc = 0; for i in 0 .. 3 { input x; let acc = acc + x; } output acc;";
    let report = prove_source_with_inputs(src, &[5, 10, 15]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![30]);
}

#[test]
fn nested_loops_unroll_by_the_product() {
    // Two nested loops of two increment an accumulator four times.
    let src = "let acc = 0;
               for i in 0 .. 2 { for j in 0 .. 2 { let acc = acc + 1; } }
               output acc;";
    let report = prove_source_with_inputs(src, &[]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![4]);
}

#[test]
fn an_empty_range_runs_the_body_zero_times() {
    // The range is empty, so the body never runs and the earlier value stands.
    let src = "let a = 3; for i in 5 .. 5 { let a = 999; } output a;";
    let report = prove_source_with_inputs(src, &[]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![3]);
}

#[test]
fn a_too_large_loop_is_a_compile_error() {
    // The range exceeds the unroll cap, so it fails fast at compile.
    let src = "for i in 0 .. 70000 { let x = 1; }";
    assert!(
        matches!(compile_source(src), Err(CompileError::LoopTooLarge)),
        "an oversized loop compiled"
    );
}
