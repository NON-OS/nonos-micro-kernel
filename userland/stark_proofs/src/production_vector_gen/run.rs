// NONOS Operating System (AGPL-3.0-or-later)
//! The gen: assemble the witness-mode recursion, commit the preprocessed
//! periodic columns, prove at deployment soundness, and emit the vector, the
//! structure, and the intermediates oracle for the contracts tree.

use super::{intermediates, structure, vector};
use crate::crypto::stark::air::{
    periodic_root, stark_prove_ext_preprocessed, stark_verify_ext_preprocessed, Air,
};
use crate::recursion_assembly::{assemble, Tamper};
use alloc::vec::Vec;

const SPEC: &str = "/Users/ek/Desktop/NOX-SmartContract/spec";

#[test]
#[ignore]
fn gen_production_recursive_vector() {
    let asm = assemble(Tamper::None);
    let wired = &asm.wired;

    let ltl = wired.log_trace_len();
    let tt = 1usize << ltl;
    let deg = wired.constraint_degree().max(1);
    let bound = (deg * tt).next_power_of_two();
    let nn = (2 * bound) << 3; // extra_blowup_bits = 3 -> rate 1/16
    let log_dn = nn.trailing_zeros();
    let fri_log_blowup = log_dn - bound.trailing_zeros();

    // The preprocessed-periodic commitment, through the same helper the prover
    // commits with and a registration recomputes: one object, not agreement.
    let n_periodic = wired.periodic_columns().len();
    let periodic_root = periodic_root(wired, 3);
    std::println!("committed {} periodic columns over 2^{}", n_periodic, log_dn);

    let off_heights: Vec<(usize, usize)> = asm
        .region_offsets
        .iter()
        .enumerate()
        .map(|(ri, &o)| {
            let end = asm.region_offsets.get(ri + 1).copied().unwrap_or(asm.lay.span);
            (o, end - o)
        })
        .collect();
    let region_transitions = wired.num_transition() - asm.n_groups;
    structure::emit(
        &asm,
        &off_heights,
        fri_log_blowup,
        log_dn,
        n_periodic,
        region_transitions,
        &periodic_root,
        &alloc::format!("{}/production-air-structure.json", SPEC),
    );

    // The deployment proof with the periodic sidecar: rate 1/16, 16 grind
    // bits = 128-bit conjectured, verified against the baked root.
    let wproof = stark_prove_ext_preprocessed(wired, &asm.witness, 32, 16, 3);
    assert!(
        stark_verify_ext_preprocessed(wired, &wproof, 32, 16, 3, &periodic_root),
        "the production recursive vector does not verify"
    );

    vector::emit(
        &asm,
        &wproof,
        fri_log_blowup,
        &alloc::format!("{}/production-recursive-vector.json", SPEC),
    );
    intermediates::emit(&asm, &wproof, &alloc::format!("{}/reference/intermediates.json", SPEC));
}
