// NONOS Operating System (AGPL-3.0-or-later)
//! The root bindings: each opening's checkpoint == the transcript-absorbed
//! root it must authenticate under. Openings 0 and 1 (FRI leaf, deep) share
//! fri.roots[0] absorbed in the FRI transcript; opening 2 is comp_root and
//! 3.. are the trace roots, absorbed in the STARK transcript.

use super::super::layout::Layout;
use super::helpers::group;
use crate::crypto::stark::air::{GpGroup, RATE};
use alloc::vec::Vec;

pub(crate) fn roots(lay: &Layout, out: &mut Vec<GpGroup>) {
    let l = lay.l;
    for (o, cell) in lay.ocells.iter().enumerate() {
        let cp_row = lay.m_off + cell.0 + lay.depth * l;
        let mut sw: Vec<(usize, usize, usize, usize)> = Vec::new();
        for j in 0..RATE {
            let arow = if o <= 1 {
                lay.ft_off + j * l
            } else if o == 2 {
                (lay.pub_len + lay.ntr * RATE + lay.ncoeff2 + j) * l
            } else {
                (lay.pub_len + (o - 3) * RATE + j) * l
            };
            sw.push((cp_row, j, arow, 8));
        }
        out.push(group(lay.span, alloc::vec![0, 1, 2, 3, 8], &sw));
    }
}
