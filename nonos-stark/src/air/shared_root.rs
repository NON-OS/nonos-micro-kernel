// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! One root for a whole batch of Merkle openings.
//!
//! Pinning a root per opening costs a boundary set per opening, so the
//! constraint count grows with the batch and proving cost with its square. A
//! capsule set opens one policy tree, so the root is a single public value: it
//! rides two periodic columns and one checkpoint constraint, at a cost that does
//! not depend on how many capsules are enrolled.

use super::super::field::{Felt, Fp};
use super::poseidon::{RATE, WIDTH};
use alloc::vec::Vec;

/// Periodic index of the checkpoint selector. The `RATE` columns after it carry
/// the selector times the root. Sits past the per-proof columns: round
/// constants, the two boundary selectors, direction, siblings, reset.
pub(super) const CKPT: usize = WIDTH + 3 + RATE + WIDTH;

/// Periodic column count for this form.
pub(super) const COLS: usize = CKPT + 1 + RATE;

/// Append one row of the checkpoint columns. `at_checkpoint` marks the row where
/// an opening's path has folded to the root; elsewhere both the selector and the
/// root columns are zero, so the constraint reads `0 = 0`.
pub(super) fn push_row(cols: &mut [Vec<Fp>], root: &[Fp; RATE], at_checkpoint: bool) {
    cols[CKPT].push(if at_checkpoint { Fp::ONE } else { Fp::ZERO });
    for (c, r) in root.iter().enumerate() {
        cols[CKPT + 1 + c].push(if at_checkpoint { *r } else { Fp::ZERO });
    }
}

/// The checkpoint constraints: on a checkpoint row the low `RATE` lanes must
/// equal the root, and off one the row is unconstrained.
pub(super) fn constraints<F: Felt>(window: &[F], periodic: &[F]) -> Vec<F> {
    let ckpt = periodic[CKPT];
    (0..RATE).map(|j| ckpt * window[j] - periodic[CKPT + 1 + j]).collect()
}
