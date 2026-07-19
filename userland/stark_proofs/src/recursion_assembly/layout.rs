// NONOS Operating System (AGPL-3.0-or-later)
//! The assembled trace geometry every binding family reads: region row
//! offsets, the padded span, and the per-region cell metadata.

use crate::crypto::stark::air::AirExt;
use alloc::boxed::Box;
use alloc::vec::Vec;

pub(crate) struct Layout {
    pub span: usize,
    /// Rounds per transcript operation.
    pub l: usize,
    pub c_off: usize,
    pub d_off: usize,
    pub ft_off: usize,
    pub f_off: usize,
    pub m_off: usize,
    pub i_off: usize,
    pub fp_off: usize,
    pub pz_off: usize,
    pub z_op: usize,
    pub deep_coeff_op: usize,
    pub pub_len: usize,
    pub ntr: usize,
    pub ncoeff2: usize,
    pub n_terms: usize,
    pub width_inner: usize,
    pub window_inner: usize,
    pub ocells: Vec<(usize, usize)>,
    pub depth: usize,
    pub n_open: usize,
    pub n_folds: usize,
    pub log_n: u32,
    pub pbits: usize,
    pub fbits: usize,
    pub t_inner: usize,
    pub n_pz: usize,
    /// The FRI query's layer-zero position, q0 mod (n / 2).
    pub i0: usize,
}

/// Each region's first row in the stacked trace, and the padded span.
pub(crate) fn offsets(regions: &[Box<dyn AirExt>]) -> (Vec<usize>, usize) {
    let mut off = Vec::with_capacity(regions.len());
    let mut r = 0usize;
    for reg in regions {
        off.push(r);
        r += 1usize << reg.log_trace_len();
    }
    (off, r.next_power_of_two())
}
