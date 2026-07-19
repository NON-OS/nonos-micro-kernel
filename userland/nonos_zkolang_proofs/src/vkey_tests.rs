// NONOS Operating System (AGPL-3.0-or-later)
//! The verifier key and its periodic root. The golden test is the strong one the
//! STARK team asked for: prove a real program with the preprocessed prover, then
//! the preprocessed verifier must accept using the root the helper computed and
//! reject a wrong one. That pins the helper's root as exactly the baked root a
//! proof needs, by the same code path, so the registration cannot drift from the
//! proof.

use nonos_stark::air::{stark_prove_ext_preprocessed, stark_verify_ext_preprocessed};
use nonos_stark::field::Fp;
use nonos_zkolang::{
    compile_source, periodic_root, registration_key, registration_root, verifier_key, StepAir, Vm,
    REGISTRATION_RATE,
};

const QUERIES: usize = 32;
const GRIND: u32 = 8;
const BLOWUP: u32 = 0;

// The trace length rounded up to a power of two, log2. Matches the driver and the
// verifier-key helper.
fn log_t(steps: usize) -> u32 {
    let mut lg = 1u32;
    while (1usize << lg) < steps {
        lg += 1;
    }
    lg
}

#[test]
fn the_helper_root_is_the_prover_baked_root() {
    let program = compile_source("input x; let y = x * x * x; output y;").expect("compile");
    let inputs = [Fp::from_u64(3)];
    let mut vm = Vm::new();
    let trace = vm.run(&program, &inputs, 1).expect("run");
    let air = StepAir::compile(
        &program,
        log_t(trace.rows.len()),
        &trace.public_inputs,
        &trace.public_outputs,
    )
    .expect("air");
    let flat = air.build_trace(&trace).expect("layout");

    let proof = stark_prove_ext_preprocessed(&air, &flat, QUERIES, GRIND, BLOWUP);
    let root = periodic_root(&program, BLOWUP).expect("root");

    assert!(
        stark_verify_ext_preprocessed(&air, &proof, QUERIES, GRIND, BLOWUP, &root),
        "the preprocessed verifier rejected the helper's periodic root"
    );

    let mut wrong = root;
    wrong[0] ^= 1;
    assert!(
        !stark_verify_ext_preprocessed(&air, &proof, QUERIES, GRIND, BLOWUP, &wrong),
        "a wrong periodic root verified"
    );
}

#[test]
fn the_verifier_key_is_deterministic_and_distinguishing() {
    let square = compile_source("input x; let y = x * x; output y;").expect("compile");
    let cube = compile_source("input x; let y = x * x * x; output y;").expect("compile");
    let k1 = verifier_key(&square, BLOWUP).expect("key");
    let k2 = verifier_key(&square, BLOWUP).expect("key");
    let k3 = verifier_key(&cube, BLOWUP).expect("key");
    assert_eq!(k1, k2, "the verifier key is not deterministic");
    assert_ne!(k1, k3, "two programs share a verifier key");
}

#[test]
fn the_rate_is_part_of_the_key() {
    // A different FRI rate is a different periodic domain, so a different key.
    let program = compile_source("input x; let y = x * x; output y;").expect("compile");
    assert_ne!(
        verifier_key(&program, 0).expect("key"),
        verifier_key(&program, 3).expect("key"),
        "the FRI rate did not change the key"
    );
}

#[test]
fn the_registration_helpers_pin_the_market_rate() {
    // The market registers and challenges through the rate-pinned helpers, so they
    // must equal the general functions evaluated at the fixed registration rate. If
    // the pinned constant ever drifted from what a proof commits, this fails.
    let program = compile_source("input x; let y = x * x * x; output y;").expect("compile");
    assert_eq!(REGISTRATION_RATE, 3, "the pinned market rate moved");
    assert_eq!(
        registration_key(&program).expect("key"),
        verifier_key(&program, REGISTRATION_RATE).expect("key"),
        "the registration key is not the key at the pinned rate"
    );
    assert_eq!(
        registration_root(&program).expect("root"),
        periodic_root(&program, REGISTRATION_RATE).expect("root"),
        "the registration root is not the root at the pinned rate"
    );
}

#[test]
fn a_proof_at_the_registration_rate_verifies_against_the_registration_root() {
    // The strongest binding at the exact rate the NOX market registers: prove a real
    // program at the registration rate, then the preprocessed verifier must accept
    // under the root the registration helper baked, and reject a one-bit flip of it.
    let program = compile_source("input x; let y = x * x * x; output y;").expect("compile");
    let inputs = [Fp::from_u64(4)];
    let mut vm = Vm::new();
    let trace = vm.run(&program, &inputs, 1).expect("run");
    let air = StepAir::compile(
        &program,
        log_t(trace.rows.len()),
        &trace.public_inputs,
        &trace.public_outputs,
    )
    .expect("air");
    let flat = air.build_trace(&trace).expect("layout");

    let rate = REGISTRATION_RATE;
    let proof = stark_prove_ext_preprocessed(&air, &flat, QUERIES, GRIND, rate);
    let root = registration_root(&program).expect("root");

    assert!(
        stark_verify_ext_preprocessed(&air, &proof, QUERIES, GRIND, rate, &root),
        "a proof at the registration rate was rejected under the registration root"
    );

    let mut wrong = root;
    wrong[0] ^= 1;
    assert!(
        !stark_verify_ext_preprocessed(&air, &proof, QUERIES, GRIND, rate, &wrong),
        "a flipped registration root verified"
    );
}
