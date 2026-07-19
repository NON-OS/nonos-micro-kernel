// NONOS Operating System (AGPL-3.0-or-later)
//! The verifier key and its periodic root. The golden test is the strong one the
//! STARK team asked for: prove a real program with the preprocessed prover, then
//! the preprocessed verifier must accept using the root the helper computed and
//! reject a wrong one. That pins the helper's root as exactly the baked root a
//! proof needs, by the same code path, so the registration cannot drift from the
//! proof.

use nonos_stark::air::{stark_prove_ext_preprocessed, stark_verify_ext_preprocessed};
use nonos_stark::field::Fp;
use nonos_zkolang::{compile_source, periodic_root, verifier_key, StepAir, Vm};

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
