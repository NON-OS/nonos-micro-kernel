// NONOS Operating System (AGPL-3.0-or-later)
//! Region 8: the inner periodic columns at z, recomputed in-region with the
//! barycentric form over the inner trace domain, so the composition's
//! periodic inputs are derived from the bound z rather than trusted.

use super::inner::Inner;
use crate::crypto::stark::air::{Air, PeriodicZ};
use crate::crypto::stark::field::Fp;
use alloc::vec::Vec;

pub(crate) fn periodic_region(inner: &Inner) -> (PeriodicZ, Vec<Fp>) {
    let region = PeriodicZ::new(
        inner.air.log_trace_len(),
        inner.g,
        inner.air.periodic_columns(),
        inner.ci.z,
    );
    let trace = region.trace();
    (region, trace)
}
