// NONOS Operating System (AGPL-3.0-or-later)
//! The consistency-index bindings: region 6's derived point is the x every
//! DEEP term divides by, and its bits are the authenticated path directions
//! of every consistency opening, so all of them open the index the point is
//! derived from. Level zero lives in the opened-cell column choice.

use super::super::layout::Layout;
use super::helpers::{chain, group};
use crate::crypto::stark::air::GpGroup;
use alloc::vec::Vec;

pub(crate) fn index(lay: &Layout, out: &mut Vec<GpGroup>) {
    let l = lay.l;
    out.push(group(
        lay.span,
        alloc::vec![1, 2, 14, 15],
        &[(lay.i_off + lay.pbits, 1, lay.d_off, 14), (lay.i_off + lay.pbits, 2, lay.d_off, 15)],
    ));

    let span5 = lay.ocells[1].0;
    let mut sw: Vec<(usize, usize, usize, usize)> = Vec::new();
    for m in 1..lay.depth {
        let mut cells: Vec<(usize, usize)> = alloc::vec![(lay.i_off + m, 0)];
        for o in 1..lay.n_open {
            cells.push((lay.m_off + o * span5 + m * l - 1, 8));
        }
        chain(&cells, &mut sw);
    }
    out.push(group(lay.span, alloc::vec![0, 8], &sw));
}
