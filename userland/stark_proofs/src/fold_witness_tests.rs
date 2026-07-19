// NONOS Operating System (AGPL-3.0-or-later)
//! The fold in witness form against the real inner proof: the per-layer
//! points, inverses, and position bits ride the trace, the points chained
//! in-region as x_(m+1) = x_m^2 * (-1)^dir_m, so the region derives its own
//! evaluation points from the layer-zero point and the bits instead of
//! trusting per-query periodic data.

use crate::crypto::stark::air::{stark_prove_ext, stark_verify_ext, TraceFoldExt};
use crate::crypto::stark::field::Fp;
use crate::crypto::stark::fri::root_of_unity;
use crate::crypto::stark::poseidon_transcript::PoseidonTranscript;
use crate::recursion_assembly::inner::{hasher, join_split, GRIND};
use alloc::vec::Vec;

#[test]
fn the_fold_witness_form_derives_its_points() {
    let h = hasher();
    let inner = join_split(&h);
    let fri = &inner.proof.fri;
    let n_folds = fri.roots.len();
    let blowup = fri.final_layer.len();
    let log_n = n_folds as u32 + blowup.trailing_zeros();
    let n = 1usize << log_n;

    let mut ts = PoseidonTranscript::new(h.clone());
    let mut betas = Vec::with_capacity(n_folds);
    for root in &fri.roots {
        ts.absorb_digest(root);
        betas.push(ts.challenge_fp2());
    }
    for value in &fri.final_layer {
        ts.absorb(value.c0);
        ts.absorb(value.c1);
    }
    assert!(ts.verify_pow(fri.pow_nonce, GRIND), "P's FRI proof-of-work did not check");
    let q0 = ts.challenge_index(n);

    let final_value = fri.final_layer[0];
    let bo = root_of_unity(log_n);
    let shift = Fp::from_u64(7);
    let (mut a, mut b) = (Vec::new(), Vec::new());
    let (mut xs, mut x_inv, mut dir) = (Vec::new(), Vec::new(), Vec::new());
    for (m, op) in fri.queries[0].layers.iter().enumerate() {
        a.push(op.a);
        b.push(op.b);
        let i = q0 % (n >> (m + 1));
        let x = (shift * bo.pow(i as u64)).pow(1u64 << m);
        xs.push(x);
        x_inv.push(x.inv());
        dir.push(i >= (n >> (m + 2)));
    }
    a.push(final_value);
    b.push(final_value);

    // The chain the region enforces reproduces the real per-layer points:
    // dropping the top index bit and squaring flips the sign exactly when the
    // bit was set.
    for m in 0..n_folds - 1 {
        let sign = if dir[m] { Fp::ZERO - Fp::ONE } else { Fp::ONE };
        assert_eq!(xs[m + 1], xs[m] * xs[m] * sign, "the point chain broke at layer {}", m);
    }
    let i0 = q0 % (n >> 1);
    assert_eq!(xs[0], shift * bo.pow(i0 as u64), "the layer-zero point is not shift*omega^i0");

    let log_layers = (n_folds + 1).next_power_of_two().trailing_zeros();
    let fold = TraceFoldExt::new_witness(log_layers, n_folds, x_inv, dir, final_value);
    let ftrace = fold.trace(&betas, &a, &b);
    let fproof = stark_prove_ext(&fold, &ftrace, 32, 8);
    assert!(
        stark_verify_ext(&fold, &fproof, 32, 8),
        "the witness-form fold chain was rejected in-circuit"
    );
}
