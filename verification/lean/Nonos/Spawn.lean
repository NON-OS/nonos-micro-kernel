/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The spawn admission invariant: a capsule runs only if its attestation verified.
The kernel gate `verify_capsule_attestation` returns `Ok` for a capsule exactly
when its proof passes, and the spawn path admits a capsule only on that `Ok`, so
whatever the sequence of spawns, every admitted capsule was attested. The gate
is `#[must_use]`, so its result cannot be dropped; `Nonos.Stark.Attest` proves
what a passing proof means (an enrolled leaf bound to that capsule). This module
proves the admission side: nothing unattested ever enters the admitted set.
`userland/kernel_proofs` and the attestation host tests discharge the gate
decision on the real code.
-/

namespace Nonos.Spawn

/-- The gate's decision for a capsule: true when its attestation verified. -/
abbrev Attested := Nat → Bool

/-- One spawn: a capsule is admitted only if it is attested, otherwise the
    spawn is rejected and the admitted set is unchanged. -/
def admit (att : Attested) (admitted : List Nat) (cap : Nat) : List Nat :=
  if att cap then cap :: admitted else admitted

/-- A whole run of spawns from an initial admitted set. -/
def run (att : Attested) (admitted : List Nat) : List Nat → List Nat
  | [] => admitted
  | cap :: rest => run att (admit att admitted cap) rest

/-- One admission preserves the invariant: if every currently admitted capsule
    is attested, so is every capsule after admitting one more. -/
theorem admit_preserves (att : Attested) (admitted : List Nat) (cap : Nat)
    (h : ∀ c ∈ admitted, att c = true) :
    ∀ c ∈ admit att admitted cap, att c = true := by
  intro c hc
  unfold admit at hc
  by_cases hcap : att cap = true
  · rw [hcap] at hc
    simp at hc
    rcases hc with hc | hc
    · rw [hc]; exact hcap
    · exact h c hc
  · simp [hcap] at hc
    exact h c hc

/-- The invariant holds along any run: from an all-attested start, every
    admitted capsule stays attested. -/
theorem run_preserves (att : Attested) (trace : List Nat) :
    ∀ (admitted : List Nat), (∀ c ∈ admitted, att c = true) →
      ∀ c ∈ run att admitted trace, att c = true := by
  induction trace with
  | nil => intro admitted h c hc; exact h c hc
  | cons cap rest ih =>
    intro admitted h c hc
    exact ih (admit att admitted cap) (admit_preserves att admitted cap h) c hc

/-- No unattested capsule ever runs: after any sequence of spawns from an empty
    start, every admitted capsule was attested. The attacker chooses the spawn
    order and never gets an unattested capsule admitted. -/
theorem only_attested_capsules_run (att : Attested) (trace : List Nat) (c : Nat)
    (h : c ∈ run att [] trace) : att c = true :=
  run_preserves att trace [] (by intro c hc; exact absurd hc (List.not_mem_nil c)) c h

end Nonos.Spawn
