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

//! The recursive FRI verifier, assembled in steps. This file holds the parts
//! that do not depend on the interactive challenge transcript or the per-query
//! verifier, so they stand alone and are proven here; the transcript-driven
//! challenge derivation and the per-query membership-and-fold checks compose in
//! as those gadgets land.
//!
//! First component: the low-degree conclusion. FRI folds a codeword down to a
//! final layer that an honest prover can only reach if the original was low
//! degree, and that final layer must be a single repeated constant. This AIR
//! proves exactly that in-circuit: the witnessed final layer is constant and
//! equals the value the proof committed. It is the last check the recursive
//! verifier makes, and the one gate a high-degree inner proof cannot pass.

use super::super::field::Fp;
use super::poseidon::{Poseidon, RATE, WIDTH};
use super::spec::Air;
use alloc::vec;
use alloc::vec::Vec;

/// Proof that a FRI final layer is the constant it committed to.
pub struct FinalLayerConstant {
    /// Log2 of the padded final-layer length.
    log_len: u32,
    /// The committed final-layer value the whole layer must equal.
    value: Fp,
}

impl FinalLayerConstant {
    pub fn new(log_len: u32, value: Fp) -> FinalLayerConstant {
        FinalLayerConstant { log_len, value }
    }

    /// The honest trace: the committed value repeated across the padded layer.
    pub fn trace(&self) -> Vec<Fp> {
        vec![self.value; 1usize << self.log_len]
    }
}

impl Air for FinalLayerConstant {
    fn log_trace_len(&self) -> u32 {
        self.log_len
    }

    fn trace_width(&self) -> usize {
        1
    }

    fn window_size(&self) -> usize {
        2
    }

    fn constraint_degree(&self) -> usize {
        1
    }

    fn num_transition(&self) -> usize {
        1
    }

    fn transition(&self, window: &[Fp], _periodic: &[Fp]) -> Vec<Fp> {
        // Each value equals its successor: the layer is constant.
        vec![window[1] - window[0]]
    }

    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        // That constant is the committed final-layer value.
        vec![(0, 0, self.value)]
    }
}

// ---------------------------------------------------------------------------
// Step 3: the transcript-driven challenge derivation.
// ---------------------------------------------------------------------------

/// The recursive verifier's Fiat-Shamir, replayed in-circuit. A FRI proof draws
/// its fold challenges and query positions from a Poseidon transcript over the
/// committed layer roots and the final layer, in a fixed schedule: for each
/// layer, absorb the root and squeeze the fold challenge; then absorb the final
/// layer; then squeeze one challenge per query. This AIR runs that exact
/// schedule over the arithmetized Poseidon permutation and pins each squeezed
/// challenge to its value, so a proof made with the real transcript has its
/// challenges re-derived here rather than trusted. The query challenges are the
/// field elements the FRI verifier masks into positions; the masking composes
/// where the query verifier consumes them.
///
/// The duplex matches `super::super::poseidon_transcript::PoseidonTranscript`:
/// state starts at zero, an absorb adds a value into lane zero and permutes, a
/// squeeze reads lane zero and permutes.
pub struct FriTranscript {
    hasher: Poseidon,
    log_rounds: u32,
    /// The public FRI layer roots, one rate-sized digest per fold.
    roots: Vec<[Fp; RATE]>,
    /// The public final layer.
    final_layer: Vec<Fp>,
    n_queries: usize,
    /// The derived fold challenges, one per layer, pinned as the squeeze output.
    betas: Vec<Fp>,
    /// The derived query challenges, before masking into positions.
    query_challenges: Vec<Fp>,
    log_trace: u32,
}

impl FriTranscript {
    /// Build the transcript for a proof with these public roots and final layer,
    /// deriving the challenges with the same permutation the AIR arithmetizes.
    pub fn new(
        hasher: Poseidon,
        log_rounds: u32,
        roots: Vec<[Fp; RATE]>,
        final_layer: Vec<Fp>,
        n_queries: usize,
    ) -> FriTranscript {
        // Replay the schedule out of circuit to obtain the pinned challenges.
        let mut state = [Fp::ZERO; WIDTH];
        let mut betas = Vec::with_capacity(roots.len());
        for root in &roots {
            for v in root.iter() {
                state[0] = state[0] + *v;
                state = hasher.permute(state);
            }
            betas.push(state[0]);
            state = hasher.permute(state);
        }
        for v in &final_layer {
            state[0] = state[0] + *v;
            state = hasher.permute(state);
        }
        let mut query_challenges = Vec::with_capacity(n_queries);
        for _ in 0..n_queries {
            query_challenges.push(state[0]);
            state = hasher.permute(state);
        }

        let ops = Self::op_count(roots.len(), final_layer.len(), n_queries);
        let rows = (ops << log_rounds).next_power_of_two().max(2);
        let log_trace = rows.trailing_zeros();

        FriTranscript {
            hasher,
            log_rounds,
            roots,
            final_layer,
            n_queries,
            betas,
            query_challenges,
            log_trace,
        }
    }

    /// The public challenges this transcript derived, for the caller to feed on.
    pub fn betas(&self) -> &[Fp] {
        &self.betas
    }

    pub fn query_challenges(&self) -> &[Fp] {
        &self.query_challenges
    }

    /// The trace rows holding the squeezed fold challenges, one per layer, in
    /// layer order. Each `beta_m` is pinned in column zero at the returned row:
    /// a squeeze reads lane zero at its operation's first row, so the challenge
    /// lives at `(row, 0)`. The wiring engine's column-zero grand product can
    /// bind it directly to the folding challenge a fold region witnesses in its
    /// own column zero, with no repositioning.
    pub fn beta_rows(&self) -> Vec<usize> {
        self.squeeze_points().into_iter().take(self.betas.len()).map(|(row, _)| row).collect()
    }

    /// The trace rows holding the squeezed query challenges, in query order,
    /// each likewise in column zero at `(row, 0)`.
    pub fn query_challenge_rows(&self) -> Vec<usize> {
        self.squeeze_points().into_iter().skip(self.betas.len()).map(|(row, _)| row).collect()
    }

    /// The number of permutations the schedule runs: for each layer, `RATE`
    /// absorbs plus one squeeze; the final-layer absorbs; and one squeeze per
    /// query.
    fn op_count(n_folds: usize, final_len: usize, n_queries: usize) -> usize {
        n_folds * (RATE + 1) + final_len + n_queries
    }

    /// The value absorbed into lane zero at the start of each operation, in
    /// order, and `Fp::ZERO` for the squeeze operations that absorb nothing.
    fn inject_schedule(&self) -> Vec<Fp> {
        let mut inject = Vec::with_capacity(Self::op_count(
            self.roots.len(),
            self.final_layer.len(),
            self.n_queries,
        ));
        for root in &self.roots {
            for v in root.iter() {
                inject.push(*v);
            }
            inject.push(Fp::ZERO); // the beta squeeze absorbs nothing
        }
        for v in &self.final_layer {
            inject.push(*v);
        }
        for _ in 0..self.n_queries {
            inject.push(Fp::ZERO); // the query squeeze absorbs nothing
        }
        inject
    }

    /// The operation rows at which a squeeze happens, paired with the pinned
    /// challenge, in schedule order. A squeeze reads lane zero at the start of
    /// its operation.
    fn squeeze_points(&self) -> Vec<(usize, Fp)> {
        let l = 1usize << self.log_rounds;
        let mut pts = Vec::with_capacity(self.betas.len() + self.query_challenges.len());
        let mut op = 0usize;
        for (i, _) in self.roots.iter().enumerate() {
            op += RATE; // the RATE absorb operations
            pts.push((op * l, self.betas[i]));
            op += 1; // the beta squeeze operation
        }
        op += self.final_layer.len();
        for c in &self.query_challenges {
            pts.push((op * l, *c));
            op += 1;
        }
        pts
    }

    /// The honest trace: one row per round, the state advancing under the
    /// permutation with the scheduled absorptions injected into lane zero.
    pub fn trace(&self) -> Vec<Fp> {
        let l = 1usize << self.log_rounds;
        let rows = 1usize << self.log_trace;
        let inject = self.inject_schedule();
        let ops = inject.len();

        let mut state = [Fp::ZERO; WIDTH];
        state[0] = inject[0]; // the first operation's absorb seeds the state
        let mut flat = Vec::with_capacity(rows * WIDTH);
        for r in 0..rows {
            flat.extend_from_slice(&state);
            let rc = self.hasher.round_constant(r % l);
            let mut nxt = self.hasher.round_with_rc(&state, &rc);
            // At an operation boundary, the next operation absorbs its value.
            if r % l == l - 1 {
                let next_op = r / l + 1;
                if next_op < ops {
                    nxt[0] = nxt[0] + inject[next_op];
                }
            }
            state = nxt;
        }
        flat
    }
}

impl Air for FriTranscript {
    fn log_trace_len(&self) -> u32 {
        self.log_trace
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
        let l = 1usize << self.log_rounds;
        let rows = 1usize << self.log_trace;
        let inject = self.inject_schedule();
        let ops = inject.len();

        // WIDTH round-constant columns, then the lane-zero injection column.
        let mut cols: Vec<Vec<Fp>> = (0..WIDTH + 1).map(|_| Vec::with_capacity(rows)).collect();
        for r in 0..rows {
            let rc = self.hasher.round_constant(r % l);
            for (j, col) in cols.iter_mut().take(WIDTH).enumerate() {
                col.push(rc[j]);
            }
            // The injection is nonzero only at an operation's last round, and
            // then only when the next operation absorbs.
            let value = if r % l == l - 1 {
                let next_op = r / l + 1;
                if next_op < ops {
                    inject[next_op]
                } else {
                    Fp::ZERO
                }
            } else {
                Fp::ZERO
            };
            cols[WIDTH].push(value);
        }
        cols
    }

    fn transition(&self, window: &[Fp], periodic: &[Fp]) -> Vec<Fp> {
        let mut state = [Fp::ZERO; WIDTH];
        state.copy_from_slice(&window[..WIDTH]);
        let mut rc = [Fp::ZERO; WIDTH];
        rc.copy_from_slice(&periodic[..WIDTH]);
        let inject = periodic[WIDTH];

        let pr = self.hasher.round_with_rc(&state, &rc);
        let mut out = Vec::with_capacity(WIDTH);
        for (j, next) in window[WIDTH..2 * WIDTH].iter().enumerate() {
            let extra = if j == 0 { inject } else { Fp::ZERO };
            out.push(*next - (pr[j] + extra));
        }
        out
    }

    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        let inject = self.inject_schedule();
        let mut b = Vec::with_capacity(WIDTH + self.betas.len() + self.query_challenges.len());
        // The state starts with the first absorbed value in lane zero, capacity
        // and the rest at zero: the sponge initialization.
        b.push((0, 0, inject[0]));
        for j in 1..WIDTH {
            b.push((j, 0, Fp::ZERO));
        }
        // Each squeeze reads lane zero at its operation's first row.
        for (row, expected) in self.squeeze_points() {
            b.push((0, row, expected));
        }
        b
    }
}

// ---------------------------------------------------------------------------
// Step 4: the recursive verifier, composed from the checked components.
// ---------------------------------------------------------------------------

use super::super::fri::root_of_unity;
use super::super::fri_poseidon::FriProof;
use super::multi_membership::{MultiMembership, Opening};
use super::types::StarkProof;
use super::verify::stark_verify;

/// The direction bits of a leaf position, one per tree level from the leaf up:
/// bit `k` is the position's `k`-th bit, which is the child side at level `k`.
fn directions(index: usize, depth: usize) -> Vec<bool> {
    (0..depth).map(|k| (index >> k) & 1 == 1).collect()
}

/// The rate-sized leaf a codeword value commits to: the value in lane zero, the
/// rest zero, matching the FRI commitment.
fn value_leaf(v: Fp) -> [Fp; RATE] {
    let mut leaf = [Fp::ZERO; RATE];
    leaf[0] = v;
    leaf
}

/// The two openings a query induces at one FRI layer: the value at the queried
/// half-position and its fold partner, each with its committed path.
pub fn layer_openings(
    proof: &FriProof,
    query: usize,
    layer: usize,
    index: usize,
    log_n: u32,
) -> Vec<Opening> {
    let half = (1usize << log_n) >> (layer + 1);
    let i = index % half;
    let op = &proof.queries[query].layers[layer];
    let root = proof.roots[layer];
    vec![
        Opening {
            leaf: value_leaf(op.a),
            root,
            siblings: op.a_path.clone(),
            directions: directions(i, op.a_path.len()),
        },
        Opening {
            leaf: value_leaf(op.b),
            root,
            siblings: op.b_path.clone(),
            directions: directions(i + half, op.b_path.len()),
        },
    ]
}

/// The membership AIR a query's layer openings must satisfy, rebuilt from the
/// proof so a verifier and a prover agree on it without trust.
pub fn layer_membership(
    proof: &FriProof,
    hasher: &Poseidon,
    log_rounds: u32,
    query: usize,
    layer: usize,
    index: usize,
    log_n: u32,
) -> MultiMembership {
    MultiMembership::new(
        hasher.clone(),
        log_rounds,
        layer_openings(proof, query, layer, index, log_n),
    )
}

/// Verify a recursion-ready FRI proof from the checked components. The fold
/// challenges are re-derived by the transcript AIR rather than trusted; the
/// Merkle openings at every query and layer are proven committed by the
/// membership AIR; the folds are the cheap public linear check, using the
/// re-derived challenges; and the final layer is a single constant the folds
/// land on. Every expensive step is a STARK, so a verifier of this verifier is
/// itself a STARK: recursion. The component proofs are supplied by the prover;
/// `transcript_proof` attests the challenges, and `membership_proofs` is one
/// proof per `(query, layer)` in row-major order.
#[allow(clippy::too_many_arguments)]
pub fn recursive_verify(
    proof: &FriProof,
    hasher: &Poseidon,
    log_rounds: u32,
    shift: Fp,
    log_n: u32,
    log_blowup: u32,
    n_queries: usize,
    stark_queries: usize,
    transcript_proof: &StarkProof,
    membership_proofs: &[StarkProof],
) -> bool {
    let n = 1usize << log_n;
    let blowup = 1usize << log_blowup;
    let n_folds = (log_n - log_blowup) as usize;

    // Structural: the proof and the supplied component proofs are the right
    // shape for these parameters.
    if proof.roots.len() != n_folds
        || proof.final_layer.len() != blowup
        || proof.queries.len() != n_queries
        || membership_proofs.len() != n_queries * n_folds
    {
        return false;
    }

    // 1. The fold and query challenges, re-derived and proven from the roots and
    // the final layer.
    let transcript = FriTranscript::new(
        hasher.clone(),
        log_rounds,
        proof.roots.clone(),
        proof.final_layer.to_vec(),
        n_queries,
    );
    if !stark_verify(&transcript, transcript_proof, stark_queries) {
        return false;
    }
    let betas = transcript.betas();
    let query_challenges = transcript.query_challenges();

    // 2. The low-degree conclusion: the final layer is a single constant.
    let final_value = proof.final_layer[0];
    for v in &proof.final_layer {
        if *v != final_value {
            return false;
        }
    }

    // 3. Each query: the openings proven committed, then the public folds land
    // on the next layer, ending on the final constant.
    let base_omega = root_of_unity(log_n);
    let inv2 = Fp::from_u64(2).inv();
    for (q, qp) in proof.queries.iter().enumerate() {
        if qp.layers.len() != n_folds {
            return false;
        }
        // The masked query position: the transcript challenge into the domain.
        let index = (query_challenges[q].value() as usize) & (n - 1);

        for m in 0..n_folds {
            // The openings at this layer are proven committed by the membership
            // AIR, rebuilt from the proof so no opened value is trusted.
            let air = layer_membership(proof, hasher, log_rounds, q, m, index, log_n);
            if !stark_verify(&air, &membership_proofs[q * n_folds + m], stark_queries) {
                return false;
            }

            // The fold, a public linear check with the re-derived challenge.
            let half = n >> (m + 1);
            let i = index % half;
            let op = &qp.layers[m];
            let x = (shift * base_omega.pow(i as u64)).pow(1u64 << m);
            let even = (op.a + op.b) * inv2;
            let odd = (op.a - op.b) * inv2 * x.inv();
            let folded = even + betas[m] * odd;

            if m + 1 < n_folds {
                let half_next = n >> (m + 2);
                let next = &qp.layers[m + 1];
                let expected = if i < half_next { next.a } else { next.b };
                if folded != expected {
                    return false;
                }
            } else if folded != final_value {
                return false;
            }
        }
    }

    true
}
