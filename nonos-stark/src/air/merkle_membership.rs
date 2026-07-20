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

//! The recursion gadget: an AIR that proves a Poseidon Merkle path, so a
//! commitment opening is checked inside a STARK. The trace is the Poseidon state
//! evolving through the path, `2^log_rounds` rows per compression. Within a
//! compression the transition is a Poseidon round; at each compression boundary
//! it takes the output digest and injects the next sibling by the index bit,
//! forming the next input. The public root is pinned at the final checkpoint and
//! the public siblings at the boundaries; the leaf stays private, so a valid
//! proof is knowledge of a leaf whose path reaches the root.

use super::super::field::{Felt, Fp, Fp2};
use super::poseidon::{Poseidon, RATE, WIDTH};
use super::spec::{Air, AirExt};
use alloc::vec::Vec;

pub struct MerkleMembership {
    hasher: Poseidon,
    log_rounds: u32,
    root: [Fp; RATE],
    /// One sibling digest per tree level, from the leaf up.
    siblings: Vec<[Fp; RATE]>,
    /// The index bit at each level: false when the node is the left child.
    directions: Vec<bool>,
}

impl MerkleMembership {
    /// Build the AIR. `hasher` must be the same Poseidon the tree committed with,
    /// with `2^log_rounds` rounds. The path depth is `siblings.len()`, any value:
    /// the slots are padded to the next power of two, the extra ones inert.
    pub fn new(
        hasher: Poseidon,
        log_rounds: u32,
        root: [Fp; RATE],
        siblings: Vec<[Fp; RATE]>,
        directions: Vec<bool>,
    ) -> MerkleMembership {
        MerkleMembership { hasher, log_rounds, root, siblings, directions }
    }

    /// The witness for a private `leaf`: run the Poseidon path, injecting each
    /// sibling at its compression boundary, and lay the state out row by row. The
    /// prover holds the leaf; the verifier never sees it.
    pub fn trace(&self, leaf: [Fp; RATE]) -> Vec<Fp> {
        let l = self.rounds();
        let depth = self.depth();
        let n = (1usize << self.log_slots()) * l;
        let mut trace = alloc::vec![Fp::ZERO; n * WIDTH];
        let mut state = inject(leaf, self.siblings[0], self.directions[0]);
        for r in 0..n {
            trace[r * WIDTH..r * WIDTH + WIDTH].copy_from_slice(&state);
            let pr = self.hasher.round_with_rc(&state, &self.hasher.round_constant(r % l));
            if r % l == l - 1 && r < depth * l {
                let m = (r + 1) / l;
                let mut node = [Fp::ZERO; RATE];
                node.copy_from_slice(&pr[..RATE]);
                state = if m < depth {
                    inject(node, self.siblings[m], self.directions[m])
                } else {
                    inject(node, [Fp::ZERO; RATE], false)
                };
            } else {
                state = pr;
            }
        }
        trace
    }

    fn rounds(&self) -> usize {
        1usize << self.log_rounds
    }

    /// The number of compressions, the path depth.
    fn depth(&self) -> usize {
        self.siblings.len()
    }

    /// Log2 of the slot count: enough slots for every compression plus the final
    /// root checkpoint, rounded up to a power of two.
    fn log_slots(&self) -> u32 {
        (self.depth() + 1).next_power_of_two().trailing_zeros()
    }

    /// The Merkle-path transition over any field: one Poseidon round, then inject
    /// the digest and the sibling by the index bit at each level boundary. Written
    /// once so the path is proven at money-grade soundness over `Fp2`.
    fn transition_impl<F: Felt>(&self, window: &[F], periodic: &[F]) -> Vec<F> {
        let mut state = [F::ZERO; WIDTH];
        state.copy_from_slice(&window[..WIDTH]);
        let mut rc = [F::ZERO; WIDTH];
        rc.copy_from_slice(&periodic[..WIDTH]);
        let bnd = periodic[WIDTH];
        let dir = periodic[WIDTH + 1];
        let sib = &periodic[WIDTH + 2..WIDTH + 2 + RATE];

        let pr = self.hasher.round_generic(&state, &rc);
        let one = F::ONE;

        let mut out = Vec::with_capacity(WIDTH);
        for (j, next) in window[WIDTH..2 * WIDTH].iter().enumerate() {
            let inj = if j < RATE {
                (one - dir) * pr[j] + dir * sib[j]
            } else {
                (one - dir) * sib[j - RATE] + dir * pr[j - RATE]
            };
            let expected = (one - bnd) * pr[j] + bnd * inj;
            out.push(*next - expected);
        }
        out
    }
}

impl AirExt for MerkleMembership {
    fn transition_ext(&self, window: &[Fp2], periodic: &[Fp2]) -> Vec<Fp2> {
        self.transition_impl(window, periodic)
    }
}

impl Air for MerkleMembership {
    fn log_trace_len(&self) -> u32 {
        self.log_rounds + self.log_slots()
    }

    fn trace_width(&self) -> usize {
        WIDTH
    }

    fn window_size(&self) -> usize {
        2
    }

    fn constraint_degree(&self) -> usize {
        7
    }

    fn num_transition(&self) -> usize {
        WIDTH
    }

    fn periodic_columns(&self) -> Vec<Vec<Fp>> {
        let l = self.rounds();
        let depth = self.depth();
        let n = 1usize << self.log_trace_len();

        // WIDTH round-constant columns, then a boundary selector, a direction,
        // and RATE sibling columns.
        let mut cols: Vec<Vec<Fp>> = (0..WIDTH + 2 + RATE).map(|_| Vec::with_capacity(n)).collect();
        for r in 0..n {
            let rc = self.hasher.round_constant(r % l);
            for (j, col) in cols.iter_mut().take(WIDTH).enumerate() {
                col.push(rc[j]);
            }
            // A boundary transition is the last round of a real compression: its
            // successor row starts the next slot.
            let is_boundary = r % l == l - 1 && r < depth * l;
            cols[WIDTH].push(if is_boundary { Fp::ONE } else { Fp::ZERO });
            // The slot the boundary forms, and the sibling and direction it uses.
            let m = (r + 1) / l;
            let (dir, sib) = if is_boundary && m < depth {
                (self.directions[m], self.siblings[m])
            } else {
                (false, [Fp::ZERO; RATE])
            };
            cols[WIDTH + 1].push(if dir { Fp::ONE } else { Fp::ZERO });
            for (c, s) in sib.iter().enumerate() {
                cols[WIDTH + 2 + c].push(*s);
            }
        }
        cols
    }

    fn transition(&self, window: &[Fp], periodic: &[Fp]) -> Vec<Fp> {
        self.transition_impl(window, periodic)
    }

    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        let l = self.rounds();
        let depth = self.depth();
        let mut b = Vec::with_capacity(WIDTH);

        // The first slot holds [leaf, sibling_0] by the first index bit; the leaf
        // half is free, the sibling half is pinned.
        let base = if self.directions[0] { 0 } else { RATE };
        for (c, s) in self.siblings[0].iter().enumerate() {
            b.push((base + c, 0, *s));
        }
        // The final checkpoint holds the root in its rate lanes.
        for (c, r) in self.root.iter().enumerate() {
            b.push((c, depth * l, *r));
        }
        b
    }
}

/// Place the node and its sibling into a fresh sponge state, ordered by the index
/// bit: the node is the left child when `right` is false, the right child otherwise.
fn inject(node: [Fp; RATE], sibling: [Fp; RATE], right: bool) -> [Fp; WIDTH] {
    let mut state = [Fp::ZERO; WIDTH];
    if !right {
        state[..RATE].copy_from_slice(&node);
        state[RATE..].copy_from_slice(&sibling);
    } else {
        state[..RATE].copy_from_slice(&sibling);
        state[RATE..].copy_from_slice(&node);
    }
    state
}
