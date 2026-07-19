// NONOS Operating System (AGPL-3.0-or-later)
//! The pay-to-prove fee model. A quote follows the proving work, splits into a
//! prover payment and a protocol cut, and the pieces are internally consistent.

use nonos_zkolang::{prove_source_with_inputs, quote};

#[test]
fn a_proof_has_a_priced_quote() {
    let report = prove_source_with_inputs("input x; let y = x * x; output y;", &[3]).expect("run");
    let q = quote(&report);
    // The cost driver is the trace area.
    assert_eq!(q.cells, (report.trace_len * report.trace_width) as u64);
    // The buyer pays base plus compute.
    assert_eq!(q.total_micronox, q.base_micronox + q.compute_micronox);
    // The fee splits exactly into the prover payment and the protocol cut.
    assert_eq!(q.prover_micronox + q.protocol_fee_micronox, q.total_micronox);
    // Both sides are paid something, and the protocol earns revenue.
    assert!(q.prover_micronox > 0, "the prover was paid nothing");
    assert!(q.protocol_fee_micronox > 0, "the protocol earned nothing");
    assert!(q.prover_micronox > q.protocol_fee_micronox, "the prover should keep the majority");
}

#[test]
fn a_larger_trace_costs_more() {
    // A longer program pads to a larger trace, so its compute component is at
    // least as large. Price follows work.
    let small = quote(&prove_source_with_inputs("input x; output x;", &[1]).expect("run"));
    let big = quote(
        &prove_source_with_inputs(
            "input x; let a = x * x; let b = a * a; let c = b * b; output c;",
            &[2],
        )
        .expect("run"),
    );
    assert!(big.total_micronox >= small.total_micronox, "a larger proof did not cost more");
}
