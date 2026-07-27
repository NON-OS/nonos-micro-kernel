// NONOS Operating System (AGPL-3.0-or-later)
//! The statement bindings: the out-of-domain point cycled through the
//! transcript squeeze, the compose input, and the periodic recompute; the
//! sixteen composition coefficients; and comp_z into the DEEP check.

use super::super::layout::Layout;
use super::helpers::{chain, group};
use crate::crypto::stark::air::{GpGroup, RATE};
use alloc::vec::Vec;

pub(crate) fn statement(lay: &Layout, out: &mut Vec<GpGroup>) {
    let l = lay.l;
    // z: squeezed in the transcript == compose's z == the periodic recompute's z.
    let mut zsw = Vec::new();
    chain(&[(lay.z_op * l, 0), (lay.c_off, 22), (lay.pz_off, 4)], &mut zsw);
    chain(&[((lay.z_op + 1) * l, 0), (lay.c_off, 23), (lay.pz_off, 5)], &mut zsw);
    out.push(group(lay.span, alloc::vec![0, 22, 23, 4, 5], &zsw));

    // The sixteen coefficient lanes, four per group, squeezed after the
    // publics and the trace-root absorbs.
    for grp in 0..4 {
        let bc = 24 + 4 * grp;
        let op = lay.pub_len + lay.ntr * RATE + 4 * grp;
        out.push(group(
            lay.span,
            alloc::vec![0, bc, bc + 1, bc + 2, bc + 3],
            &[
                (op * l, 0, lay.c_off, bc),
                ((op + 1) * l, 0, lay.c_off, bc + 1),
                ((op + 2) * l, 0, lay.c_off, bc + 2),
                ((op + 3) * l, 0, lay.c_off, bc + 3),
            ],
        ));
    }

    // comp_z: the composition value == the DEEP check's composition claim.
    out.push(group(
        lay.span,
        alloc::vec![54, 55, 4, 5],
        &[(lay.c_off, 54, lay.d_off, 4), (lay.c_off, 55, lay.d_off, 5)],
    ));
}
