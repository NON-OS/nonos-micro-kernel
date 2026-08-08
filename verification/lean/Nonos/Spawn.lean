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

/-! ### What the gate actually decides

The theorems above take `att` as given, so they propagate an invariant without
saying where it comes from. `attest_gate` supplies it, and it has three
branches and two build configurations, one of which admits regardless. Leaving
that out is what let the result above read as an unconditional claim about the
running kernel. -/

/-- The three ways `attest_gate` ends, branch for branch: an empty attestation
    trailer, a trailer `verify_capsule_attestation` accepted, and one it
    rejected. -/
inductive Outcome where
  | noTrailer
  | verified
  | rejected
  deriving DecidableEq, Repr

/-- The configurations the gate compiles into. `nonos-zk-rollout` turns both
    refusals into log-and-continue; without it the gate refuses. -/
inductive Mode where
  | enforcing
  | rollout
  deriving DecidableEq, Repr

/-- The gate's decision, mirroring the `cfg` arms in `attest_gate`. -/
def gate : Mode → Outcome → Bool
  | .enforcing, .verified => true
  | .enforcing, _ => false
  | .rollout, _ => true

/-- Which mode a feature combination compiles to. `none` is the combination
    that does not compile at all: `src/lib.rs` rejects production together with
    rollout through `compile_error!`, on the grounds that production builds
    must enforce attestation rather than log a failed proof. -/
def modeOf (production rollout : Bool) : Option Mode :=
  match production, rollout with
  | true, true => none
  | _, true => some .rollout
  | _, false => some .enforcing

/-- The attestation predicate the gate supplies, given what each capsule's
    trailer verified to. -/
def attOf (m : Mode) (outcome : Nat → Outcome) : Attested :=
  fun c => gate m (outcome c)

/-- Enforcing: a capsule is admitted only when its attestation verified. -/
theorem enforcing_admits_only_verified (o : Outcome) (h : gate .enforcing o = true) :
    o = .verified := by
  cases o <;> simp [gate] at h ⊢

/-- Enforcing: a missing trailer is refused. Worth naming because it is the
    branch that runs before any verification happens, and the one an unsigned
    capsule takes. -/
theorem enforcing_refuses_a_missing_trailer : gate .enforcing .noTrailer = false := rfl

/-- Enforcing: a failed proof is refused. -/
theorem enforcing_refuses_a_failed_proof : gate .enforcing .rejected = false := rfl

/-- **The gate property.** In an enforcing build, after any sequence of spawns,
    every capsule that ran had a trailer that verified. Unlike
    `only_attested_capsules_run` this does not take the predicate as given: it
    is the gate's own decision, so a branch returning `Ok` where it should
    refuse makes this false. -/
theorem enforcing_run_admits_only_verified (outcome : Nat → Outcome)
    (trace : List Nat) (c : Nat) (h : c ∈ run (attOf .enforcing outcome) [] trace) :
    outcome c = .verified :=
  enforcing_admits_only_verified (outcome c)
    (only_attested_capsules_run (attOf .enforcing outcome) trace c h)

/-- Rollout admits every capsule, including one with no trailer at all. Stated
    rather than omitted: under `nonos-zk-rollout` the gate is not a gate, and a
    model that quietly assumed otherwise would claim a property the build does
    not have. -/
theorem rollout_admits_everything (o : Outcome) : gate .rollout o = true := by
  cases o <;> rfl

/-- Which is why production cannot be a rollout build. Every combination that
    compiles with `nonos-production` set is enforcing, so the gate property
    holds of every production kernel. -/
theorem production_is_always_enforcing (rollout : Bool) (m : Mode)
    (h : modeOf true rollout = some m) : m = .enforcing := by
  cases rollout with
  | true => simp [modeOf] at h
  | false => simp [modeOf] at h; exact h.symm

end Nonos.Spawn
