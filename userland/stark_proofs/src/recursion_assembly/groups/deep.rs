// NONOS Operating System (AGPL-3.0-or-later)
//! The DEEP bindings: the check's z and coefficients to the transcript, its
//! batched result and composition value to their authenticated openings,
//! every trace value to its authenticated opening, and the claims cycled
//! through the composition frame and the transcript-absorbed frame.

use super::super::layout::Layout;
use super::helpers::{cycle, group};
use crate::crypto::stark::air::GpGroup;
use alloc::vec::Vec;

pub(crate) fn deep(lay: &Layout, out: &mut Vec<GpGroup>) {
    let l = lay.l;
    // The DEEP z == the transcript's squeezed out-of-domain point.
    out.push(group(
        lay.span,
        alloc::vec![0, 10, 11],
        &[(lay.z_op * l, 0, lay.d_off + 1, 10), ((lay.z_op + 1) * l, 0, lay.d_off + 1, 11)],
    ));
    // The batched result == the authenticated DEEP opening.
    let (dr, dc) = (lay.m_off + lay.ocells[1].0, lay.ocells[1].1);
    out.push(group(
        lay.span,
        alloc::vec![2, 3, dc, dc + 1],
        &[(lay.d_off + lay.n_terms, 2, dr, dc), (lay.d_off + lay.n_terms, 3, dr, dc + 1)],
    ));
    // The composition term's value == the authenticated composition opening.
    let (cr, cc) = (lay.m_off + lay.ocells[2].0, lay.ocells[2].1);
    out.push(group(
        lay.span,
        alloc::vec![6, 7, cc, cc + 1],
        &[(lay.d_off + lay.n_terms - 1, 6, cr, cc), (lay.d_off + lay.n_terms - 1, 7, cr, cc + 1)],
    ));

    // Every trace value feeding the batch == its authenticated opening, one
    // cycle over the window copies plus the leaf; the imaginary lane rides a
    // second cycle so a base value cannot smuggle an extension part.
    for c in 0..lay.width_inner {
        let leaf_row = lay.m_off + lay.ocells[3 + c].0;
        let leaf_col = lay.ocells[3 + c].1;
        for lane in 0..2 {
            let mut cells: Vec<(usize, usize)> = (0..lay.window_inner)
                .map(|k| (lay.d_off + k * lay.width_inner + c, 6 + lane))
                .collect();
            cells.push((leaf_row, leaf_col + lane));
            out.push(cycle(lay.span, &cells));
        }
    }

    // Each batching coefficient == its transcript squeeze.
    for i in 0..lay.n_terms {
        let op = lay.deep_coeff_op + 2 * i;
        out.push(group(
            lay.span,
            alloc::vec![0, 12, 13],
            &[(op * l, 0, lay.d_off + i, 12), ((op + 1) * l, 0, lay.d_off + i, 13)],
        ));
    }

    // The claims == the composition frame == the transcript-absorbed frame.
    for i in 0..6 {
        let ood_c0 = lay.z_op + 2 + 2 * i;
        out.push(cycle(lay.span, &[(lay.c_off, 2 * i), (lay.d_off + i, 8), (ood_c0 * l, 8)]));
        out.push(cycle(
            lay.span,
            &[(lay.c_off, 2 * i + 1), (lay.d_off + i, 9), ((ood_c0 + 1) * l, 8)],
        ));
    }
}
