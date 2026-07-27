// NONOS Operating System (AGPL-3.0-or-later)
//! Group construction: a group's sigma is the identity over its cells with the
//! bound cells swapped, so identity cells cancel and only the explicit swaps
//! (or chained cycles) must hold equal.

use crate::crypto::stark::air::GpGroup;
use crate::crypto::stark::field::Fp;
use alloc::vec::Vec;

pub(crate) fn group(
    span: usize,
    wcols: Vec<usize>,
    swaps: &[(usize, usize, usize, usize)],
) -> GpGroup {
    let k = wcols.len();
    let mut sigma: Vec<usize> = (0..span * k).collect();
    for &(ra, ca, rb, cb) in swaps {
        let ia = wcols.iter().position(|&c| c == ca).unwrap();
        let ib = wcols.iter().position(|&c| c == cb).unwrap();
        sigma.swap(ra * k + ia, rb * k + ib);
    }
    GpGroup { wired_cols: wcols, sigma, beta: Fp::from_u64(5), gamma: Fp::from_u64(7) }
}

/// Chain consecutive cells into transpositions: composed, a cycle forcing all
/// of them equal.
pub(crate) fn chain(cells: &[(usize, usize)], out: &mut Vec<(usize, usize, usize, usize)>) {
    for w in cells.windows(2) {
        out.push((w[0].0, w[0].1, w[1].0, w[1].1));
    }
}

/// One group holding one cycle of equal cells.
pub(crate) fn cycle(span: usize, cells: &[(usize, usize)]) -> GpGroup {
    let mut wcols: Vec<usize> = cells.iter().map(|c| c.1).collect();
    wcols.sort_unstable();
    wcols.dedup();
    let mut swaps = Vec::new();
    chain(cells, &mut swaps);
    group(span, wcols, &swaps)
}
