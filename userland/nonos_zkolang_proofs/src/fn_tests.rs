// NONOS Operating System (AGPL-3.0-or-later)
//! Functions, inlined at compile time. A function is a hygienic macro with call
//! syntax: its body sees only its parameters, and each call is its body with the
//! arguments substituted. There is no call stack and no recursion, so a program
//! stays straight-line and proves the same way.

use nonos_zkolang::{compile_source, prove_source_with_inputs, CompileError};

#[test]
fn a_function_is_inlined_at_its_call() {
    let src = "fn sq(x) = x * x; input a; let y = sq(a); output y;";
    let report = prove_source_with_inputs(src, &[5]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![25]);
}

#[test]
fn a_function_cuts_repetition() {
    // The sum of two squares, each written once.
    let src = "fn sq(x) = x * x; input a; input b; let r = sq(a) + sq(b); output r;";
    let report = prove_source_with_inputs(src, &[3, 4]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![25]);
}

#[test]
fn functions_nest() {
    // quad(x) = sq(sq(x)) = x^4.
    let src = "fn sq(x) = x * x; fn quad(x) = sq(sq(x)); input a; output quad(a);";
    let report = prove_source_with_inputs(src, &[2]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![16]);
}

#[test]
fn a_multi_argument_function() {
    // madd(a, b, c) = a*b + c.
    let src = "fn madd(a, b, c) = a * b + c; input x; output madd(x, x, 1);";
    let report = prove_source_with_inputs(src, &[6]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![37], "6*6 + 1");
}

#[test]
fn a_function_body_is_hygienic() {
    // The body's `x` is the parameter, not the caller's `x = 100`.
    let src = "fn inc(x) = x + 1; let x = 100; input a; let y = inc(a); output y;";
    let report = prove_source_with_inputs(src, &[5]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![6], "the caller's x leaked into the body");
}

#[test]
fn a_free_variable_in_a_body_is_an_error() {
    // A body may reference only its parameters and other functions.
    let src = "fn g(x) = x + z; input a; output g(a);";
    assert!(matches!(compile_source(src), Err(CompileError::UnknownVariable)));
}

#[test]
fn an_unknown_function_is_an_error() {
    assert!(matches!(compile_source("output nope(3);"), Err(CompileError::UnknownFunction)));
}

#[test]
fn an_arity_mismatch_is_an_error() {
    let src = "fn f(x) = x; output f(1, 2);";
    assert!(matches!(compile_source(src), Err(CompileError::ArityMismatch)));
}

#[test]
fn recursion_is_an_error() {
    let src = "fn f(x) = f(x); output f(1);";
    assert!(matches!(compile_source(src), Err(CompileError::RecursionTooDeep)));
}
