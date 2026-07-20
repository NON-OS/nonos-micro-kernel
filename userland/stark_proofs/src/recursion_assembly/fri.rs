// NONOS Operating System (AGPL-3.0-or-later)
//! Regions 3 and 4: the FRI transcript in witness form and the fold chain in
//! witness form. The transcript replays the root absorbs and beta squeezes;
//! the fold carries the per-layer points, inverses, and position bits the
//! square-and-sign chain derives from the query index.

use super::inner::{Inner, GRIND};
use super::sponge::{absorb, squeeze};
use crate::crypto::stark::air::{Poseidon, TraceFoldExt, TranscriptCheck, TranscriptOp, WIDTH};
use crate::crypto::stark::field::{Fp, Fp2};
use crate::crypto::stark::fri::root_of_unity;
use crate::crypto::stark::poseidon_transcript::PoseidonTranscript;
use alloc::vec::Vec;

pub(crate) struct FriSide {
    pub transcript: TranscriptCheck,
    pub ttrace: Vec<Fp>,
    pub fold: TraceFoldExt,
    pub ftrace: Vec<Fp>,
    pub n_folds: usize,
    pub log_n: u32,
    /// The layer-zero position of query zero: q0 mod (n / 2).
    pub i0: usize,
}

pub(crate) fn fri_side(h: &Poseidon, inner: &Inner) -> FriSide {
    let fri = &inner.proof.fri;
    let n_folds = fri.roots.len();
    let blowup = fri.final_layer.len();
    let log_n = n_folds as u32 + blowup.trailing_zeros();
    let n = 1usize << log_n;

    let mut fs = PoseidonTranscript::new(h.clone());
    let mut betas: Vec<Fp2> = Vec::with_capacity(n_folds);
    let mut st = [Fp::ZERO; WIDTH];
    let mut ops: Vec<TranscriptOp> = Vec::new();
    for root in &fri.roots {
        fs.absorb_digest(root);
        betas.push(fs.challenge_fp2());
        for lane in root {
            absorb(h, &mut ops, &mut st, *lane);
        }
        squeeze(h, &mut ops, &mut st);
        squeeze(h, &mut ops, &mut st);
    }
    for value in &fri.final_layer {
        fs.absorb(value.c0);
        fs.absorb(value.c1);
    }
    assert!(fs.verify_pow(fri.pow_nonce, GRIND), "the FRI proof-of-work did not check");
    let q0 = fs.challenge_index(n);
    let transcript = TranscriptCheck::new_witness(h.clone(), 2, ops);
    let ttrace = transcript.trace();

    let final_value = fri.final_layer[0];
    let bo = root_of_unity(log_n);
    let shift = Fp::from_u64(7);
    let (mut a, mut b, mut xi, mut dir) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    for (m, op) in fri.queries[0].layers.iter().enumerate() {
        a.push(op.a);
        b.push(op.b);
        let ix = q0 % (n >> (m + 1));
        xi.push((shift * bo.pow(ix as u64)).pow(1u64 << m).inv());
        dir.push(ix >= (n >> (m + 2)));
    }
    a.push(final_value);
    b.push(final_value);
    let log_layers = (n_folds + 1).next_power_of_two().trailing_zeros();
    let fold = TraceFoldExt::new_witness(log_layers, n_folds, xi, dir, final_value);
    let ftrace = fold.trace(&betas, &a, &b);

    FriSide { transcript, ttrace, fold, ftrace, n_folds, log_n, i0: q0 % (n >> 1) }
}
