// NONOS Operating System (AGPL-3.0-or-later)
//! Region 2: the DEEP consistency check in witness form. The terms and the
//! evaluation point ride the trace; the tampers that must be rejected are a
//! trace value cut loose from its authenticated opening and a batching
//! coefficient off the transcript, both internally consistent in-region so
//! only the binding catches them.

use super::inner::{Inner, EXTRA};
use super::tamper::Tamper;
use crate::crypto::stark::air::{deep_terms_query0_pub, DeepCheckExt, Poseidon};
use crate::crypto::stark::field::{Fp, Fp2};
use alloc::vec::Vec;

pub(crate) fn deep_region(
    h: &Poseidon,
    inner: &Inner,
    tamper: Tamper,
) -> (DeepCheckExt, Vec<Fp>, usize) {
    let (mut terms, dx, ddeep) =
        deep_terms_query0_pub(&inner.air, &inner.proof, EXTRA, h, &inner.publics);
    match tamper {
        Tamper::ReboundTraceValue => terms[0].val = terms[0].val + Fp2::ONE,
        Tamper::OffTranscriptCoeff => terms[0].coeff = terms[0].coeff + Fp2::ONE,
        _ => {}
    }
    let n_terms = terms.len();
    let region = DeepCheckExt::new_witness(terms, dx, ddeep);
    let trace = region.trace();
    (region, trace, n_terms)
}
