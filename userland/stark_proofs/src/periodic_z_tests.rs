// NONOS Operating System (AGPL-3.0-or-later)
//! The periodic-z gadget against the real inner proof: the in-circuit
//! barycentric recompute agrees with the verifier's native evaluation and
//! proves standalone.

use crate::crypto::stark::air::{stark_prove_ext, stark_verify_ext, Air, PeriodicZ};
use crate::recursion_assembly::inner::{hasher, join_split};

#[test]
fn the_periodic_z_region_matches_the_native_evaluation() {
    let h = hasher();
    let inner = join_split(&h);
    let pz = PeriodicZ::new(
        inner.air.log_trace_len(),
        inner.g,
        inner.air.periodic_columns(),
        inner.ci.z,
    );
    assert_eq!(
        pz.values(),
        inner.ci.periodic_z,
        "the barycentric recompute disagrees with the native evaluation"
    );
    let trace = pz.trace();
    let proof = stark_prove_ext(&pz, &trace, 32, 8);
    assert!(stark_verify_ext(&pz, &proof, 32, 8), "the periodic-z recompute was rejected");
}
