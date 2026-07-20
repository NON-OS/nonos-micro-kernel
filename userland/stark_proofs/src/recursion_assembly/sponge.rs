// NONOS Operating System (AGPL-3.0-or-later)
//! The duplex schedule both transcript regions replay: an absorb adds into
//! lane zero and permutes, a squeeze reads lane zero and permutes.

use crate::crypto::stark::air::{Poseidon, TranscriptOp, WIDTH};
use crate::crypto::stark::field::Fp;
use alloc::vec::Vec;

pub(crate) fn absorb(h: &Poseidon, ops: &mut Vec<TranscriptOp>, st: &mut [Fp; WIDTH], val: Fp) {
    ops.push(TranscriptOp::Absorb(val));
    st[0] = st[0] + val;
    *st = h.permute(*st);
}

pub(crate) fn squeeze(h: &Poseidon, ops: &mut Vec<TranscriptOp>, st: &mut [Fp; WIDTH]) {
    ops.push(TranscriptOp::Squeeze(st[0]));
    *st = h.permute(*st);
}
