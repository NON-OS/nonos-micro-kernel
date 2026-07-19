// NONOS Operating System (AGPL-3.0-or-later)
//! Soundness of public input and output binding. An honest run with a public
//! input and output verifies; substituting a different public value that the
//! prover did not actually compute against must be rejected. These build the
//! AIR directly so the public values handed to the verifier can be forged, which
//! the driver's honest path never does.

use nonos_stark::air::{stark_prove_poseidon_ext, stark_verify_poseidon_ext, Poseidon, RATE};
use nonos_stark::field::Fp;
use nonos_zkolang::{compile_source, StepAir, Vm};

const QUERIES: usize = 32;
const GRIND: u32 = 16;
const BLOWUP: u32 = 3;
const LOG_T: u32 = 4;

fn hasher() -> Poseidon {
    Poseidon::new(2, [Fp::ZERO; RATE])
}

// Build the honest trace of `input x; let y = x*x; output y;` on x, returning the
// VM trace so a test can hand the verifier forged public values.
fn square_trace(x: u64) -> nonos_zkolang::Trace {
    let program = compile_source("input x; let y = x * x; output y;").expect("compile");
    let inputs = [Fp::from_u64(x)];
    let mut vm = Vm::new();
    vm.run(&program, &inputs, 1).expect("run")
}

fn verifies_against(public_inputs: &[Fp], public_outputs: &[Fp]) -> bool {
    let program = compile_source("input x; let y = x * x; output y;").expect("compile");
    let trace = square_trace(3);
    let air = StepAir::compile(&program, LOG_T, public_inputs, public_outputs).expect("air");
    let flat = air.build_trace(&trace).expect("layout");
    let h = hasher();
    let proof = stark_prove_poseidon_ext(&air, &flat, QUERIES, GRIND, BLOWUP, &h);
    stark_verify_poseidon_ext(&air, &proof, QUERIES, GRIND, BLOWUP, &h)
}

#[test]
fn honest_public_io_verifies() {
    // x = 3, y = 9: the values the trace was actually built from.
    assert!(verifies_against(&[Fp::from_u64(3)], &[Fp::from_u64(9)]), "honest public io rejected");
}

#[test]
fn forged_public_input_is_rejected() {
    // The trace ran x = 3, but the verifier is told x = 4. The input boundary
    // binds the read to the committed value, so this must fail.
    assert!(
        !verifies_against(&[Fp::from_u64(4)], &[Fp::from_u64(9)]),
        "a forged public input verified"
    );
}

#[test]
fn forged_public_output_is_rejected() {
    // The trace computed y = 9, but the verifier is told the output is 10.
    assert!(
        !verifies_against(&[Fp::from_u64(3)], &[Fp::from_u64(10)]),
        "a forged public output verified"
    );
}
