// NONOS Operating System (AGPL-3.0-or-later)
//! The fold bindings: the betas to their transcript squeezes, the layer-zero
//! opened value to the authenticated FRI leaf, and the point provenance: the
//! layer-zero x seeded by region 7's product chain, whose bits are the fold's
//! own position bits and the leaf opening's path directions.

use super::super::layout::Layout;
use super::helpers::{chain, group};
use crate::crypto::stark::air::GpGroup;
use alloc::vec::Vec;

pub(crate) fn fold(lay: &Layout, out: &mut Vec<GpGroup>) {
    let l = lay.l;
    let mut bsw: Vec<(usize, usize, usize, usize)> = Vec::new();
    for m in 0..lay.n_folds {
        bsw.push((lay.ft_off + (6 * m + 4) * l, 0, lay.f_off + m, 0));
        bsw.push((lay.ft_off + (6 * m + 5) * l, 0, lay.f_off + m, 1));
    }
    out.push(group(lay.span, alloc::vec![0, 1], &bsw));

    let (mr, mc) = (lay.m_off + lay.ocells[0].0, lay.ocells[0].1);
    out.push(group(
        lay.span,
        alloc::vec![2, 3, mc, mc + 1],
        &[(lay.f_off, 2, mr, mc), (lay.f_off, 3, mr, mc + 1)],
    ));

    // The layer-zero point == region 7's derived shift * omega^i0, so the
    // square-and-sign chain descends from the query index.
    out.push(group(lay.span, alloc::vec![1, 6], &[(lay.fp_off + lay.fbits, 1, lay.f_off, 6)]));

    // Bit k == the fold's direction at layer log_n - 2 - k (the fold consumes
    // the index top-down) == the leaf opening's path direction at level k.
    // Level zero lives in the opened-cell column choice.
    let mut fsw: Vec<(usize, usize, usize, usize)> = Vec::new();
    for k in 0..lay.fbits {
        let mut cells: Vec<(usize, usize)> = alloc::vec![(lay.fp_off + k, 0)];
        if k >= 1 {
            cells.push((lay.m_off + k * l - 1, 8));
        }
        let fold_row = lay.log_n as usize - 2 - k;
        if fold_row < lay.n_folds {
            cells.push((lay.f_off + fold_row, 8));
        }
        chain(&cells, &mut fsw);
    }
    out.push(group(lay.span, alloc::vec![0, 8], &fsw));
}
