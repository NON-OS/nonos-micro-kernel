/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The security guarantee of the post-quantum capsule attestation, composed from
the two pieces proven elsewhere. An attestation is a Poseidon Merkle membership
proof (Nonos.Stark.Merkle) carried under a Fiat-Shamir challenge bound to the
capsule context (Nonos.Stark.Transcript). It is accepted against the kernel's
trusted root and the spawning capsule's context exactly when the leaf recomputes
to that root and the proof's challenge is the one that context derives.

From that, an accepted attestation names an enrolled leaf, and it is bound to a
single capsule: a proof drawn for one context cannot admit another, and the root
pins which leaf was enrolled. The two standing assumptions are made explicit as
hypotheses: pairwise injectivity of the compression (collision resistance of the
real Poseidon hash) and injectivity of the context binding (Fiat-Shamir
soundness). `userland/stark_proofs` discharges this on the real gate operation in
`air_tests.rs`: `an_enrolled_capsule_attestation_is_accepted`,
`attestation_is_denied_for_a_different_capsule_or_root`, and
`a_real_attestation_trailer_verifies_and_tampering_is_rejected`.
-/

import Nonos.Stark.Merkle

namespace Nonos.Stark.Attest

open Nonos.Stark.Merkle

variable {α : Type}

/-- An attestation: the private leaf, its public path, and the challenge the
    proof was drawn under. -/
structure Attestation (α : Type) where
  leaf : α
  path : List (Step α)
  challenge : Nat

/-- The kernel's acceptance: the leaf recomputes to the trusted `root` under the
    compression `f`, and the proof's challenge is the one the capsule `ctx`
    derives through `bind`. Both conditions are the gate's own checks. -/
def accepts (f : α → α → α) (bind : Nat → Nat) (a : Attestation α) (root : α)
    (ctx : Nat) : Prop :=
  recompute f a.leaf a.path = root ∧ a.challenge = bind ctx

/-- An accepted attestation names an enrolled leaf: its path reaches the trusted
    root. This is the membership half. -/
theorem accepted_leaf_is_enrolled (f : α → α → α) (bind : Nat → Nat)
    (a : Attestation α) (root : α) (ctx : Nat)
    (h : accepts f bind a root ctx) : recompute f a.leaf a.path = root :=
  h.1

/-- An attestation is bound to one capsule: the same proof cannot be accepted
    under two different contexts, given the context binding is injective (the
    Fiat-Shamir soundness assumption). -/
theorem bound_to_one_capsule (f : α → α → α) (bind : Nat → Nat)
    (hbind : ∀ x y, bind x = bind y → x = y)
    (a : Attestation α) (root : α) (ctx ctx' : Nat)
    (h : accepts f bind a root ctx) (h' : accepts f bind a root ctx') :
    ctx = ctx' :=
  hbind ctx ctx' (h.2 ▸ h'.2)

/-- A proof drawn for one capsule is rejected for another: non-replayability,
    the property that stops an enrolled capsule from vouching for a different
    spawn. -/
theorem a_proof_for_one_capsule_is_rejected_for_another (f : α → α → α)
    (bind : Nat → Nat) (hbind : ∀ x y, bind x = bind y → x = y)
    (a : Attestation α) (root : α) (ctx ctx' : Nat) (hne : ctx ≠ ctx')
    (h : accepts f bind a root ctx) : ¬ accepts f bind a root ctx' :=
  fun h' => hne (bound_to_one_capsule f bind hbind a root ctx ctx' h h')

/-- The root pins the enrolled leaf: two attestations accepted against the same
    root along the same path must be of the same leaf, given the compression is
    pairwise injective (collision resistance). Enrollment in some other tree, or
    a substituted leaf, does not pass. -/
theorem the_root_pins_the_leaf (f : α → α → α) (bind : Nat → Nat)
    (hf : ∀ a b a' b', f a b = f a' b' → a = a' ∧ b = b')
    (a a' : Attestation α) (root : α) (ctx : Nat) (hpath : a.path = a'.path)
    (h : accepts f bind a root ctx) (h' : accepts f bind a' root ctx) :
    a.leaf = a'.leaf := by
  apply distinct_leaves_give_distinct_roots f hf a.path
  rw [h.1, hpath, h'.1]

end Nonos.Stark.Attest
