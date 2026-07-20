// NONOS Operating System (AGPL-3.0-or-later)
//! Region 1: the out-of-domain composition check in witness form. The frame,
//! periodic values, point, coefficients, and composition ride the trace,
//! bound by the assembly to their sources.

use super::inner::Inner;
use crate::crypto::stark::air::{Air, ComposeBoundary, ComposeCheck};
use crate::crypto::stark::field::{Fp, Fp2};
use alloc::vec::Vec;

pub(crate) fn compose_region(inner: &Inner) -> (ComposeCheck, Vec<Fp>) {
    let mut window = [Fp2::ZERO; 6];
    window.copy_from_slice(&inner.proof.ood_frame[..6]);
    let mut cp = [Fp2::ZERO; 5];
    cp.copy_from_slice(&inner.ci.periodic_z[..5]);
    let mut cf = [Fp2::ZERO; 8];
    cf.copy_from_slice(&inner.ci.coeffs[..8]);
    let bnds: Vec<ComposeBoundary> = inner
        .air
        .boundary()
        .iter()
        .map(|(col, row, e)| ComposeBoundary {
            col: *col,
            g_row: inner.g.pow(*row as u64),
            expected: *e,
        })
        .collect();
    let region = ComposeCheck::new_witness(
        window,
        cp,
        cf,
        inner.ci.z,
        inner.ci.comp_z,
        inner.g.pow(inner.t - 1),
        inner.t,
        bnds,
    );
    let trace = region.trace();
    (region, trace)
}
