// NONOS Operating System (AGPL-3.0-or-later)
//! End-to-end proofs of the zkolang source language. A program written as text is
//! compiled, run, proven, and verified through the one-call driver. A true
//! program verifies; a false claim has no trace to prove; a malformed program is
//! a typed compile error. These run on the host under `cargo test`.

use nonos_zkolang::{
    compile_source, prove_source, prove_source_with_inputs, CompileError, RunError,
};

#[test]
fn source_program_proves_and_verifies() {
    let src = "
        let a = 3;
        let b = 5;
        let s = a + b;      // 8
        let p = s * s;      // 64
        assert p - 64;      // p == 64
    ";
    let report = prove_source(src).expect("an honest program failed to run");
    assert!(report.verified, "an honest program did not verify");
    assert!(report.steps > 0, "the run recorded no steps");
}

#[test]
fn equality_and_select_prove() {
    // e = (5 == 5) = 1; pick = e ? a : b = a = 3; pick - 3 == 0.
    let src = "
        let a = 3;
        let b = 5;
        let e = b == 5;
        let pick = sel(e, a, b);
        assert pick - 3;
    ";
    let report = prove_source(src).expect("an honest program failed to run");
    assert!(report.verified, "an equality-and-select program did not verify");
}

#[test]
fn inverse_proves() {
    // q = inv(7); 7 * q == 1, so (7*q) - 1 == 0.
    let src = "
        let x = 7;
        let q = inv(x);
        let one = x * q;
        assert one - 1;
    ";
    let report = prove_source(src).expect("an honest program failed to run");
    assert!(report.verified, "an inverse program did not verify");
}

#[test]
fn public_cube_proves_and_returns_output() {
    // Prove y = x^3 for a public input x = 3, exposing y as a public output.
    let src = "
        input x;
        let y = x * x * x;
        output y;
    ";
    let report = prove_source_with_inputs(src, &[3]).expect("cube failed to run");
    assert!(report.verified, "the public cube did not verify");
    assert_eq!(report.outputs, vec![27], "the public output was wrong");
}

#[test]
fn public_input_drives_the_computation() {
    // The same program on a different public input yields a different output.
    let src = "input x; let y = x * x; output y;";
    let report = prove_source_with_inputs(src, &[9]).expect("square failed to run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![81]);
}

#[test]
fn false_assertion_has_no_proof() {
    // s = 8; asserting s - 9 == 0 is false, so the run is unprovable.
    let src = "
        let a = 3;
        let b = 5;
        let s = a + b;
        assert s - 9;
    ";
    match prove_source(src) {
        Err(RunError::Execute(_)) => {}
        other => panic!("expected an unprovable run, got {other:?}"),
    }
}

#[test]
fn unknown_variable_is_a_compile_error() {
    match compile_source("let x = y + 1;") {
        Err(CompileError::UnknownVariable) => {}
        other => panic!("expected an unknown-variable error, got {other:?}"),
    }
}

#[test]
fn a_stray_character_is_a_lex_error() {
    match compile_source("let x = 3 $ 4;") {
        Err(CompileError::UnexpectedChar { .. }) => {}
        other => panic!("expected a lex error, got {other:?}"),
    }
}
