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

//! Verify several Poseidon Merkle openings in one STARK, the heavy half of a FRI
//! query verifier: a query opens two values per layer, all under different
//! roots, and this proves the whole batch at once. Each opening occupies a fixed
//! run of rows and is a Merkle path verification; at an opening boundary the
//! state resets to the next opening's public leaf and first sibling. The leaves
//! are public (a FRI proof reveals its openings), and every root is pinned at
//! its checkpoint.

use super::super::field::{Felt, Fp, Fp2};
use super::poseidon::{Poseidon, RATE, WIDTH};
use super::shared_root;
use super::spec::{Air, AirExt};
use alloc::vec::Vec;

/// One opening: a public leaf digest, its committed root, and the sibling path.
pub struct Opening {
    pub leaf: [Fp; RATE],
    pub root: [Fp; RATE],
    pub siblings: Vec<[Fp; RATE]>,
    pub directions: Vec<bool>,
}

pub struct MultiMembership {
    hasher: Poseidon,
    log_rounds: u32,
    depth: usize,
    openings: Vec<Opening>,
    witness_path: bool,
    shared_root: bool,
    /// The one root the checkpoint enforces, in the shared-root form.
    root: [Fp; RATE],
}

impl MultiMembership {
    /// Build the AIR for a batch of equal-depth openings under `hasher`. The
    /// siblings, directions, and the roots ride the periodic columns and boundaries:
    /// instance-specific structure, fine for a per-proof AIR.
    pub fn new(hasher: Poseidon, log_rounds: u32, openings: Vec<Opening>) -> MultiMembership {
        Self::build(hasher, log_rounds, openings, false, false, [Fp::ZERO; RATE])
    }

    /// Openings that all authenticate against one root, as a capsule set does.
    /// `new` pins each opening's root as its own boundary, so cost grows with the
    /// square of the batch; here one periodic checkpoint constraint enforces the
    /// shared root at a cost independent of it.
    ///
    /// `root` is the public statement and the only root enforced, so a caller
    /// cannot quietly prove a batch against a root the verifier is not checking.
    pub fn new_shared_root(
        hasher: Poseidon,
        log_rounds: u32,
        root: [Fp; RATE],
        openings: Vec<Opening>,
    ) -> MultiMembership {
        Self::build(hasher, log_rounds, openings, false, true, root)
    }

    /// The production form: the sibling and direction of each compression ride the
    /// trace (a boolean-constrained direction plus RATE sibling columns), so the AIR
    /// is instance-independent (round constants, the slot and opening selectors, and
    /// the reset column are the only periodic columns; no boundary is pinned). The
    /// opened leaf is bound by the assembly grand product to where the fold consumes
    /// it, and the root at the checkpoint is bound to the transcript-absorbed root,
    /// so the path authenticates the fold value against the committed root.
    pub fn new_witness(
        hasher: Poseidon,
        log_rounds: u32,
        openings: Vec<Opening>,
    ) -> MultiMembership {
        Self::build(hasher, log_rounds, openings, true, false, [Fp::ZERO; RATE])
    }

    fn build(
        hasher: Poseidon,
        log_rounds: u32,
        openings: Vec<Opening>,
        witness_path: bool,
        shared_root: bool,
        root: [Fp; RATE],
    ) -> MultiMembership {
        let depth = openings.first().map(|o| o.siblings.len()).unwrap_or(0);
        MultiMembership { hasher, log_rounds, depth, openings, witness_path, shared_root, root }
    }

    fn rounds(&self) -> usize {
        1usize << self.log_rounds
    }

    /// Slots per opening: a compression per level plus the root checkpoint,
    /// rounded to a power of two.
    fn log_slots(&self) -> u32 {
        (self.depth + 1).next_power_of_two().trailing_zeros()
    }

    /// Openings padded to a power of two.
    fn log_batch(&self) -> u32 {
        self.openings.len().next_power_of_two().max(1).trailing_zeros()
    }

    /// Rows per opening.
    fn span(&self) -> usize {
        (1usize << self.log_slots()) * self.rounds()
    }

    fn initial_state(&self, opening: &Opening) -> [Fp; WIDTH] {
        inject(opening.leaf, opening.siblings[0], opening.directions[0])
    }

    /// The witness trace for this batch: run each opening's Merkle path, reset to
    /// the next opening's leaf at each opening boundary, and let padding rows run
    /// the permutation. The single source of truth for the layout, so callers
    /// prove against exactly what the constraints check.
    pub fn trace(&self) -> Vec<Fp> {
        let l = self.rounds();
        let span = self.span();
        let depth = self.depth;
        let count = self.openings.len();
        let n = 1usize << self.log_trace_len();
        let w = self.trace_width();

        let mut trace = alloc::vec![Fp::ZERO; n * w];
        let mut state = self.initial_state(&self.openings[0]);
        for r in 0..n {
            trace[r * w..r * w + WIDTH].copy_from_slice(&state);
            let pr = self.hasher.round_with_rc(&state, &self.hasher.round_constant(r % l));
            let opening = r / span;
            let within = r % span;
            let at_row_bnd = within % l == l - 1;
            let is_op_bnd = within == span - 1 && opening + 1 < count;
            let is_slot_bnd = at_row_bnd && within < depth * l && !is_op_bnd;
            // In the production form the compression's direction and sibling ride the
            // trace, at exactly the rows the transition reads them.
            if self.witness_path {
                let m = (within + 1) / l;
                let (dir, sib) = if is_slot_bnd && opening < count && m < depth {
                    (self.openings[opening].directions[m], self.openings[opening].siblings[m])
                } else {
                    (false, [Fp::ZERO; RATE])
                };
                trace[r * w + WIDTH] = if dir { Fp::ONE } else { Fp::ZERO };
                for (c, s) in sib.iter().enumerate() {
                    trace[r * w + WIDTH + 1 + c] = *s;
                }
            }
            if is_op_bnd {
                state = self.initial_state(&self.openings[opening + 1]);
            } else if is_slot_bnd {
                let m = (within + 1) / l;
                let mut digest = [Fp::ZERO; RATE];
                digest.copy_from_slice(&pr[..RATE]);
                if opening < count && m < depth {
                    let o = &self.openings[opening];
                    state = inject(digest, o.siblings[m], o.directions[m]);
                } else {
                    state = inject(digest, [Fp::ZERO; RATE], false);
                }
            } else {
                state = pr;
            }
        }
        trace
    }

    /// The `(row, column)` of each opening's committed scalar in the trace, one
    /// per opening. A Poseidon-committed FRI leaf is `[value, 0, 0, 0]`, so the
    /// scalar is lane zero of the leaf, which the first index bit places in the
    /// low half of the initial state (column zero) or the high half (column
    /// `RATE`). A wiring engine binds these cells to where a fold consumes the
    /// opened value, so the fold runs on exactly what the opening committed.
    pub fn opened_cells(&self) -> alloc::vec::Vec<(usize, usize)> {
        let span = self.span();
        self.openings
            .iter()
            .enumerate()
            .map(|(o, opening)| {
                let col = if opening.directions[0] { RATE } else { 0 };
                (o * span, col)
            })
            .collect()
    }
}

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

impl MultiMembership {
    fn transition_impl<F: Felt>(&self, window: &[F], periodic: &[F]) -> Vec<F> {
        let stride = self.trace_width();
        let mut state = [F::ZERO; WIDTH];
        state.copy_from_slice(&window[..WIDTH]);
        let mut rc = [F::ZERO; WIDTH];
        rc.copy_from_slice(&periodic[..WIDTH]);
        let slot_bnd = periodic[WIDTH];
        let op_bnd = periodic[WIDTH + 1];
        // The direction and sibling ride the periodic columns in the per-proof form
        // and the trace in the production form; the reset column stays periodic
        // (structurally zero for a single opening).
        let mut sib = [F::ZERO; RATE];
        let mut reset = [F::ZERO; WIDTH];
        let dir;
        if self.witness_path {
            dir = window[WIDTH];
            sib.copy_from_slice(&window[WIDTH + 1..WIDTH + 1 + RATE]);
            reset.copy_from_slice(&periodic[WIDTH + 2..WIDTH + 2 + WIDTH]);
        } else {
            dir = periodic[WIDTH + 2];
            sib.copy_from_slice(&periodic[WIDTH + 3..WIDTH + 3 + RATE]);
            reset.copy_from_slice(&periodic[WIDTH + 3 + RATE..WIDTH + 3 + RATE + WIDTH]);
        }

        let pr = self.hasher.round_generic(&state, &rc);
        let one = F::ONE;

        let mut out = Vec::with_capacity(WIDTH + 1);
        for (j, next) in window[stride..stride + WIDTH].iter().enumerate() {
            let slot_inject = if j < RATE {
                (one - dir) * pr[j] + dir * sib[j]
            } else {
                (one - dir) * sib[j - RATE] + dir * pr[j - RATE]
            };
            let expected =
                op_bnd * reset[j] + slot_bnd * slot_inject + (one - op_bnd - slot_bnd) * pr[j];
            out.push(*next - expected);
        }
        // The witnessed direction must be a bit, so it cannot blend the two children.
        if self.witness_path {
            out.push(dir * (one - dir));
        }
        if self.shared_root {
            out.extend(shared_root::constraints(window, periodic));
        }
        out
    }
}

impl AirExt for MultiMembership {
    fn transition_ext(&self, window: &[Fp2], periodic: &[Fp2]) -> Vec<Fp2> {
        self.transition_impl(window, periodic)
    }
}

impl Air for MultiMembership {
    fn log_trace_len(&self) -> u32 {
        self.log_batch() + self.log_slots() + self.log_rounds
    }

    fn trace_width(&self) -> usize {
        if self.witness_path {
            WIDTH + 1 + RATE
        } else {
            WIDTH
        }
    }

    fn window_size(&self) -> usize {
        2
    }

    fn constraint_degree(&self) -> usize {
        8
    }

    fn num_transition(&self) -> usize {
        if self.witness_path {
            WIDTH + 1
        } else if self.shared_root {
            WIDTH + RATE
        } else {
            WIDTH
        }
    }

    fn periodic_columns(&self) -> Vec<Vec<Fp>> {
        let l = self.rounds();
        let span = self.span();
        let depth = self.depth;
        let n = 1usize << self.log_trace_len();
        let count = self.openings.len();

        // Per-proof: rc[WIDTH], slot_bnd, op_bnd, dir, sib[RATE], reset[WIDTH].
        // Production: rc[WIDTH], slot_bnd, op_bnd, reset[WIDTH] (dir and sib are trace).
        let cols_len = if self.witness_path {
            WIDTH + 2 + WIDTH
        } else if self.shared_root {
            shared_root::COLS
        } else {
            WIDTH + 3 + RATE + WIDTH
        };
        let shared = self.root;
        let mut cols: Vec<Vec<Fp>> = (0..cols_len).map(|_| Vec::with_capacity(n)).collect();

        for r in 0..n {
            let rc = self.hasher.round_constant(r % l);
            for (j, col) in cols.iter_mut().take(WIDTH).enumerate() {
                col.push(rc[j]);
            }

            let opening = r / span; // which opening this row belongs to
            let within = r % span; // row inside the opening
            let at_row_boundary = within % l == l - 1;
            let is_op_boundary = within == span - 1 && opening + 1 < count;
            // A slot boundary that injects a sibling: last round of a real
            // compression, not the opening's final reset.
            let is_slot_boundary = at_row_boundary && within < depth * l;

            cols[WIDTH].push(if is_slot_boundary && !is_op_boundary { Fp::ONE } else { Fp::ZERO });
            cols[WIDTH + 1].push(if is_op_boundary { Fp::ONE } else { Fp::ZERO });

            // Reset state to the next opening's initial, at an opening boundary
            // (structurally zero for a single opening).
            let reset = if is_op_boundary && opening + 1 < count {
                self.initial_state(&self.openings[opening + 1])
            } else {
                [Fp::ZERO; WIDTH]
            };

            if self.witness_path {
                for (c, v) in reset.iter().enumerate() {
                    cols[WIDTH + 2 + c].push(*v);
                }
            } else {
                // Sibling and direction for the slot injection at `within`.
                let m = (within + 1) / l;
                let (dir, sib) =
                    if is_slot_boundary && !is_op_boundary && opening < count && m < depth {
                        (self.openings[opening].directions[m], self.openings[opening].siblings[m])
                    } else {
                        (false, [Fp::ZERO; RATE])
                    };
                cols[WIDTH + 2].push(if dir { Fp::ONE } else { Fp::ZERO });
                for (c, s) in sib.iter().enumerate() {
                    cols[WIDTH + 3 + c].push(*s);
                }
                for (c, v) in reset.iter().enumerate() {
                    cols[WIDTH + 3 + RATE + c].push(*v);
                }
                if self.shared_root {
                    // The row where this opening's path has folded to the root.
                    let at_ckpt = within == depth * l && opening < count;
                    shared_root::push_row(&mut cols, &shared, at_ckpt);
                }
            }
        }
        cols
    }

    fn transition(&self, window: &[Fp], periodic: &[Fp]) -> Vec<Fp> {
        self.transition_impl(window, periodic)
    }

    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        let l = self.rounds();
        let span = self.span();
        let depth = self.depth;

        // Production form: nothing is pinned. The opened leaf is bound by the
        // assembly to the fold, and the root at the checkpoint is bound to the
        // transcript-absorbed root, so both are witness rather than public.
        if self.witness_path {
            return Vec::new();
        }

        let mut b = Vec::with_capacity(WIDTH + self.openings.len() * RATE);
        // The first opening's whole initial state is public.
        let first = self.initial_state(&self.openings[0]);
        for (j, v) in first.iter().enumerate() {
            b.push((j, 0, *v));
        }
        // With one shared root the checkpoint constraint enforces it on every
        // opening, so there is nothing left to pin per opening.
        if self.shared_root {
            return b;
        }
        // Every opening's root sits at its checkpoint.
        for (o, opening) in self.openings.iter().enumerate() {
            let row = o * span + depth * l;
            for (c, r) in opening.root.iter().enumerate() {
                b.push((c, row, *r));
            }
        }
        b
    }
}
