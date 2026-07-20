// NONOS Operating System (AGPL-3.0-or-later)
//! Region 0: the STARK transcript in witness form. Publics first, then the
//! proof's own sequence, continued through the out-of-domain frame and the
//! DEEP coefficient squeezes so every challenge the other regions consume
//! exists as a bindable squeeze cell.

use super::inner::Inner;
use super::sponge::{absorb, squeeze};
use crate::crypto::stark::air::{Poseidon, TranscriptCheck, TranscriptOp, WIDTH};
use crate::crypto::stark::field::Fp;
use alloc::vec::Vec;

pub(crate) struct StarkTranscript {
    pub region: TranscriptCheck,
    pub trace: Vec<Fp>,
    /// The operation squeezing z.c0; z.c1 follows it.
    pub z_op: usize,
    /// The operation squeezing the first DEEP coefficient lane.
    pub deep_coeff_op: usize,
}

pub(crate) fn stark_transcript(h: &Poseidon, inner: &Inner, n_terms: usize) -> StarkTranscript {
    let mut st = [Fp::ZERO; WIDTH];
    let mut ops: Vec<TranscriptOp> = Vec::new();
    for &p in &inner.publics {
        absorb(h, &mut ops, &mut st, p);
    }
    for root in &inner.proof.trace_roots {
        for lane in root {
            absorb(h, &mut ops, &mut st, *lane);
        }
    }
    for _ in 0..inner.ci.coeffs.len() * 2 {
        squeeze(h, &mut ops, &mut st);
    }
    for lane in &inner.proof.comp_root {
        absorb(h, &mut ops, &mut st, *lane);
    }
    let z_op = ops.len();
    squeeze(h, &mut ops, &mut st);
    squeeze(h, &mut ops, &mut st);
    for value in &inner.proof.ood_frame {
        absorb(h, &mut ops, &mut st, value.c0);
        absorb(h, &mut ops, &mut st, value.c1);
    }
    let deep_coeff_op = ops.len();
    for _ in 0..n_terms {
        squeeze(h, &mut ops, &mut st);
        squeeze(h, &mut ops, &mut st);
    }
    let region = TranscriptCheck::new_witness(h.clone(), 2, ops);
    let trace = region.trace();
    StarkTranscript { region, trace, z_op, deep_coeff_op }
}
