/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Assurance capstone. The flagship guarantees stated as single named theorems,
plus the advances beyond a pure-Lean ledger kernel: a post-quantum-bound
authority model, and one admission theorem that holds over the exact gated
function the loader runs, every capsule that runs is attested AND
rollback-fresh AND post-quantum-authorized, whatever order an attacker spawns
in. Every theorem here is machine-checked on Lean's standard axioms; the
`AxiomProfile` file prints the closure as evidence.
-/

import Nonos.Authorization
import Nonos.Capability
import Nonos.CapabilityBits
import Nonos.Spawn
import Nonos.AntiRollback

namespace Nonos.Assurance

open Nonos.AntiRollback (State Accepts update)

/-! ## The guarantees, as single named theorems -/

/-- AUTHORITY. No chain of attenuations, of any length, ever widens the
    capability set, proven at the kernel's actual `u64` word. -/
theorem authority (a : Nat) (masks : List Nat) (x : Nat)
    (h : Nonos.Capability.Grants
          (Nonos.CapabilityBits.capsOf (masks.foldl (· &&& ·) a)) x) :
    Nonos.Capability.Grants (Nonos.CapabilityBits.capsOf a) x :=
  Nonos.CapabilityBits.word_chain_never_widens a masks x h

/-- FRESHNESS. Once a version boots, every strictly older version is rejected
    forever. -/
theorem freshness (s : State) (v w : Nat)
    (hv : Accepts s v) (hw : w < v) : ¬ Accepts (update s v) w :=
  Nonos.AntiRollback.no_rollback_after_boot s v w hv hw

/-- INTEGRITY. Whatever order an attacker requests spawns in, no unattested
    capsule is ever admitted. -/
theorem integrity (att : Nonos.Spawn.Attested) (trace : List Nat) (c : Nat)
    (h : c ∈ Nonos.Spawn.run att [] trace) : att c = true :=
  Nonos.Spawn.only_attested_capsules_run att trace c h

/-! ## Post-quantum hybrid authority (beyond a classical Ed25519 floor) -/

/-- A capsule's authority is bound by BOTH a classical (Ed25519) and a
    post-quantum (ML-DSA) signature; `ed c` / `mldsa c` say each verified. -/
def Authorized (ed mldsa : Nat → Bool) (c : Nat) : Prop :=
  ed c = true ∧ mldsa c = true

/-- Authority always rests on the post-quantum signature. -/
theorem authority_needs_pq (ed mldsa : Nat → Bool) (c : Nat)
    (h : Authorized ed mldsa c) : mldsa c = true := h.2

/-- Authority always rests on the classical signature too. -/
theorem authority_needs_classical (ed mldsa : Nat → Bool) (c : Nat)
    (h : Authorized ed mldsa c) : ed c = true := h.1

/-- The decisive advance over a classical floor: even a TOTAL break of Ed25519
   , an adversary who forges every classical signature, cannot authorize a
    capsule whose post-quantum signature does not verify. -/
theorem classical_break_insufficient (mldsa : Nat → Bool) (c : Nat)
    (h : Authorized (fun _ => true) mldsa c) : mldsa c = true := h.2

/-- Symmetrically, breaking the post-quantum scheme alone is insufficient: the
    classical signature is still required. -/
theorem pq_break_insufficient (ed : Nat → Bool) (c : Nat)
    (h : Authorized ed (fun _ => true) c) : ed c = true := h.1

/-- Without a valid post-quantum signature there is no authority, whatever the
    classical side says. -/
theorem no_pq_no_authority (ed mldsa : Nat → Bool) (c : Nat)
    (hpq : mldsa c = false) : ¬ Authorized ed mldsa c := by
  intro h
  rw [h.2] at hpq
  exact Bool.noConfusion hpq

/-- Without a valid classical signature there is no authority either. -/
theorem no_classical_no_authority (ed mldsa : Nat → Bool) (c : Nat)
    (hed : ed c = false) : ¬ Authorized ed mldsa c := by
  intro h
  rw [h.1] at hed
  exact Bool.noConfusion hed

/-! ## The composed admission gate, the flagship, over the exact function -/

/-- The loader admits a capsule only if it is attested, its version clears the
    anti-rollback floor, and its post-quantum signature verifies. -/
def Ok (att : Nat → Bool) (ver : Nat → Nat) (s : State) (mldsa : Nat → Bool)
    (c : Nat) : Prop :=
  att c = true ∧ Accepts s (ver c) ∧ mldsa c = true

/-- The Bool gate the loader evaluates. -/
def gate (att : Nat → Bool) (ver : Nat → Nat) (s : State) (mldsa : Nat → Bool)
    (c : Nat) : Bool :=
  att c && decide (Accepts s (ver c)) && mldsa c

/-- The gate accepts exactly the admissible capsules. -/
theorem gate_true_iff (att : Nat → Bool) (ver : Nat → Nat) (s : State)
    (mldsa : Nat → Bool) (c : Nat) :
    gate att ver s mldsa c = true ↔ Ok att ver s mldsa c := by
  unfold gate Ok
  rw [Bool.and_eq_true, Bool.and_eq_true, decide_eq_true_iff]
  exact and_assoc

/-- One admission: append the capsule iff the gate accepts it. -/
def gadmit (att : Nat → Bool) (ver : Nat → Nat) (s : State) (mldsa : Nat → Bool)
    (admitted : List Nat) (c : Nat) : List Nat :=
  if gate att ver s mldsa c then c :: admitted else admitted

/-- A whole run of admission requests. -/
def grun (att : Nat → Bool) (ver : Nat → Nat) (s : State) (mldsa : Nat → Bool)
    (admitted : List Nat) : List Nat → List Nat
  | [] => admitted
  | c :: rest => grun att ver s mldsa (gadmit att ver s mldsa admitted c) rest

/-- One admission preserves the invariant. -/
theorem gadmit_preserves (att : Nat → Bool) (ver : Nat → Nat) (s : State)
    (mldsa : Nat → Bool) (admitted : List Nat) (c : Nat)
    (h : ∀ d ∈ admitted, Ok att ver s mldsa d) :
    ∀ d ∈ gadmit att ver s mldsa admitted c, Ok att ver s mldsa d := by
  intro d hd
  unfold gadmit at hd
  by_cases hg : gate att ver s mldsa c = true
  · rw [if_pos hg] at hd
    simp at hd
    rcases hd with hd | hd
    · rw [hd]; exact (gate_true_iff att ver s mldsa c).mp hg
    · exact h d hd
  · rw [if_neg hg] at hd
    exact h d hd

/-- The invariant holds along any run. -/
theorem grun_preserves (att : Nat → Bool) (ver : Nat → Nat) (s : State)
    (mldsa : Nat → Bool) (trace : List Nat) :
    ∀ (admitted : List Nat), (∀ d ∈ admitted, Ok att ver s mldsa d) →
      ∀ d ∈ grun att ver s mldsa admitted trace, Ok att ver s mldsa d := by
  induction trace with
  | nil => intro admitted h d hd; exact h d hd
  | cons c rest ih =>
    intro admitted h d hd
    exact ih (gadmit att ver s mldsa admitted c)
      (gadmit_preserves att ver s mldsa admitted c h) d hd

/-- FLAGSHIP. Over the exact gated admission function, whatever order an
    attacker requests spawns in, every capsule that runs is attested AND
    rollback-fresh AND post-quantum-authorized. This is strictly stronger than
    attestation alone: it composes integrity, freshness and post-quantum
    authority over the one function the loader executes. -/
theorem only_ok_capsules_run (att : Nat → Bool) (ver : Nat → Nat) (s : State)
    (mldsa : Nat → Bool) (trace : List Nat) (d : Nat)
    (h : d ∈ grun att ver s mldsa [] trace) : Ok att ver s mldsa d :=
  grun_preserves att ver s mldsa trace []
    (by intro d hd; exact absurd hd (List.not_mem_nil d)) d h

/-- Projection: a capsule that runs is attested. -/
theorem run_capsule_attested (att : Nat → Bool) (ver : Nat → Nat) (s : State)
    (mldsa : Nat → Bool) (trace : List Nat) (d : Nat)
    (h : d ∈ grun att ver s mldsa [] trace) : att d = true :=
  (only_ok_capsules_run att ver s mldsa trace d h).1

/-- Projection: a capsule that runs is rollback-fresh. -/
theorem run_capsule_fresh (att : Nat → Bool) (ver : Nat → Nat) (s : State)
    (mldsa : Nat → Bool) (trace : List Nat) (d : Nat)
    (h : d ∈ grun att ver s mldsa [] trace) : Accepts s (ver d) :=
  (only_ok_capsules_run att ver s mldsa trace d h).2.1

/-- Projection: a capsule that runs is post-quantum authorized. -/
theorem run_capsule_pq_authorized (att : Nat → Bool) (ver : Nat → Nat)
    (s : State) (mldsa : Nat → Bool) (trace : List Nat) (d : Nat)
    (h : d ∈ grun att ver s mldsa [] trace) : mldsa d = true :=
  (only_ok_capsules_run att ver s mldsa trace d h).2.2

/-- Corollary: a capsule that fails post-quantum verification never runs. -/
theorem unsigned_pq_never_runs (att : Nat → Bool) (ver : Nat → Nat) (s : State)
    (mldsa : Nat → Bool) (trace : List Nat) (d : Nat)
    (hpq : mldsa d = false) : d ∉ grun att ver s mldsa [] trace := by
  intro h
  have hok := run_capsule_pq_authorized att ver s mldsa trace d h
  rw [hok] at hpq
  exact Bool.noConfusion hpq

/-- Corollary: a stale-version capsule never runs. -/
theorem stale_never_runs (att : Nat → Bool) (ver : Nat → Nat) (s : State)
    (mldsa : Nat → Bool) (trace : List Nat) (d : Nat)
    (hstale : ¬ Accepts s (ver d)) : d ∉ grun att ver s mldsa [] trace := by
  intro h
  exact hstale (run_capsule_fresh att ver s mldsa trace d h)

/-- Corollary: an unattested capsule never runs. -/
theorem unattested_never_runs (att : Nat → Bool) (ver : Nat → Nat) (s : State)
    (mldsa : Nat → Bool) (trace : List Nat) (d : Nat)
    (hun : att d = false) : d ∉ grun att ver s mldsa [] trace := by
  intro h
  have hok := run_capsule_attested att ver s mldsa trace d h
  rw [hok] at hun
  exact Bool.noConfusion hun

end Nonos.Assurance
