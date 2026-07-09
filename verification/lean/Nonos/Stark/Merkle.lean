/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification of Merkle opening verification. A STARK commits to trace data
by a Merkle root; an opening reveals a leaf and the sibling path, and the
verifier recomputes the root and compares. This module proves the exact
acceptance behaviour: verification succeeds precisely when the recomputed
root equals the commitment, an honest opening always verifies, and, whenever
the compression function is pairwise injective, different leaves under the
same path force different roots. Collision resistance of the real compression
(BLAKE3) is not provable and is carried as an explicit hypothesis here and as
a trusted primitive in the architecture document; everything else is
unconditional. `userland/stark_proofs` discharges these on the real
`src/crypto/stark/merkle` code in `merkle_tests.rs`:
`honest_openings_always_verify`, `a_tampered_leaf_is_rejected`,
`a_tampered_path_or_root_is_rejected`, and
`distinct_leaf_sets_give_distinct_roots`.
-/

namespace Nonos.Stark.Merkle

variable {α : Type}

/-- One path entry: the sibling digest and whether the running node sits on
    the right of the pair. -/
structure Step (α : Type) where
  sibling : α
  onRight : Bool

/-- Recompute the root from a leaf up a sibling path, compressing in the
    order the direction bit dictates, exactly as the verifier does. -/
def recompute (f : α → α → α) (leaf : α) : List (Step α) → α
  | [] => leaf
  | s :: rest =>
    recompute f (if s.onRight then f s.sibling leaf else f leaf s.sibling) rest

/-- The verifier's acceptance: the recomputed root equals the commitment. -/
def verifies (f : α → α → α) (leaf : α) (path : List (Step α)) (root : α) : Prop :=
  recompute f leaf path = root

/-- Completeness: an opening taken from the committed tree verifies against
    its own root. -/
theorem an_honest_opening_verifies (f : α → α → α) (leaf : α)
    (path : List (Step α)) : verifies f leaf path (recompute f leaf path) :=
  rfl

/-- Acceptance is exactly recomputation: the verifier accepts if and only if
    the leaf and path recompute to the committed root. There is no other way
    in, so tampering with the root is rejected by definition. -/
theorem acceptance_iff_recomputation (f : α → α → α) (leaf : α)
    (path : List (Step α)) (root : α) :
    verifies f leaf path root ↔ recompute f leaf path = root :=
  Iff.rfl

/-- A leaf or path whose recomputation misses the commitment is rejected. -/
theorem a_mismatched_recomputation_is_rejected (f : α → α → α) (leaf : α)
    (path : List (Step α)) (root : α)
    (h : recompute f leaf path ≠ root) : ¬ verifies f leaf path root :=
  h

/-- Binding, conditional on the compression: if the compression function is
    pairwise injective, two different leaves cannot recompute to the same
    root along the same path. For the real BLAKE3 compression, pairwise
    injectivity up to feasible computation is collision resistance, the
    assumed primitive; under that assumption an attacker cannot swap a
    committed leaf without moving the root. -/
theorem distinct_leaves_give_distinct_roots (f : α → α → α)
    (hinj : ∀ a b a' b', f a b = f a' b' → a = a' ∧ b = b')
    (path : List (Step α)) (l l' : α)
    (h : recompute f l path = recompute f l' path) : l = l' := by
  induction path generalizing l l' with
  | nil => exact h
  | cons s rest ih =>
    have hnode := ih _ _ h
    cases hs : s.onRight with
    | true =>
      rw [hs] at hnode
      simp at hnode
      exact (hinj _ _ _ _ hnode).2
    | false =>
      rw [hs] at hnode
      simp at hnode
      exact (hinj _ _ _ _ hnode).1

end Nonos.Stark.Merkle
