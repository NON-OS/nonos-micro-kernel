// NONOS Operating System (AGPL-3.0-or-later)
//! The periodic bindings: each recomputed P_j(z), on its run's last row, is
//! the compose input cell that consumes it, so the composition runs on the
//! derived evaluations rather than prover inputs.

use super::super::layout::Layout;
use super::helpers::group;
use crate::crypto::stark::air::GpGroup;
use alloc::vec::Vec;

pub(crate) fn periodic(lay: &Layout, out: &mut Vec<GpGroup>) {
    for j in 0..lay.n_pz {
        let r = lay.pz_off + (j + 1) * lay.t_inner - 1;
        out.push(group(
            lay.span,
            alloc::vec![10, 11, 12 + 2 * j, 13 + 2 * j],
            &[(r, 10, lay.c_off, 12 + 2 * j), (r, 11, lay.c_off, 13 + 2 * j)],
        ));
    }
}
