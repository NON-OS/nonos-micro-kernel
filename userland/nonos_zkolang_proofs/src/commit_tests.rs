// NONOS Operating System (AGPL-3.0-or-later)
//! The program commitment and the public-statement binding. A proof carries a
//! stable commitment to its program, and a proof bound to one statement is
//! rejected under another: the Fiat-Shamir seeding of the public statement does
//! real work, which is what a recursive verifier and the on-chain market rely on.

use nonos_stark::air::{
    stark_prove_poseidon_ext_pub, stark_verify_poseidon_ext_pub, Poseidon, RATE,
};
use nonos_stark::field::Fp;
use nonos_zkolang::{commit, commit_limbs, compile_source, prove_source_with_inputs, StepAir, Vm};

const QUERIES: usize = 32;
const GRIND: u32 = 16;
const BLOWUP: u32 = 3;
const LOG_T: u32 = 4;

#[test]
fn commitment_is_deterministic_and_distinguishing() {
    let square = compile_source("input x; let y = x * x; output y;").expect("compile");
    let cube = compile_source("input x; let y = x * x * x; output y;").expect("compile");
    assert_eq!(commit(&square), commit(&square), "the commitment is not deterministic");
    assert_ne!(commit(&square), commit(&cube), "two programs share a commitment");
}

#[test]
fn the_report_carries_the_program_commitment() {
    let src = "input x; let y = x * x; output y;";
    let report = prove_source_with_inputs(src, &[9]).expect("run");
    assert!(report.verified);
    let expected = commit(&compile_source(src).expect("compile"));
    assert_eq!(report.program_commit, expected, "the report commitment does not match");
}

#[test]
fn a_proof_is_rejected_under_a_forged_statement() {
    let program = compile_source("input x; let y = x * x; output y;").expect("compile");
    let inputs = [Fp::from_u64(3)];
    let mut vm = Vm::new();
    let trace = vm.run(&program, &inputs, 1).expect("run");
    let air = StepAir::compile(&program, LOG_T, &trace.public_inputs, &trace.public_outputs)
        .expect("air");
    let flat = air.build_trace(&trace).expect("layout");
    let h = Poseidon::new(2, [Fp::ZERO; RATE]);

    // The true bound statement: program commitment, trace length, inputs, outputs.
    let mut publics: Vec<Fp> = Vec::new();
    publics.extend_from_slice(&commit_limbs(&program));
    publics.push(Fp::from_u64(1u64 << LOG_T));
    publics.extend_from_slice(&trace.public_inputs);
    publics.extend_from_slice(&trace.public_outputs);

    let proof = stark_prove_poseidon_ext_pub(&air, &flat, QUERIES, GRIND, BLOWUP, &h, &publics);
    assert!(
        stark_verify_poseidon_ext_pub(&air, &proof, QUERIES, GRIND, BLOWUP, &h, &publics),
        "an honestly bound proof was rejected"
    );

    // Forge the program commitment the verifier is told; the transcript diverges.
    let mut forged = publics.clone();
    forged[0] = forged[0] + Fp::ONE;
    assert!(
        !stark_verify_poseidon_ext_pub(&air, &proof, QUERIES, GRIND, BLOWUP, &h, &forged),
        "a proof verified under a forged program commitment"
    );

    // Forge the bound trace length; the fee claim would no longer hold.
    let mut wrong_len = publics.clone();
    let len_slot = 4;
    wrong_len[len_slot] = wrong_len[len_slot] + Fp::ONE;
    assert!(
        !stark_verify_poseidon_ext_pub(&air, &proof, QUERIES, GRIND, BLOWUP, &h, &wrong_len),
        "a proof verified under a forged trace length"
    );
}
