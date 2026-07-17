/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Context binding for the attestation transcript. The context (the capsule identity,
the boot epoch) is absorbed before anything else, so the challenge depends on it.
The theorems below show the challenge is a function of context and data, a proof
drawn under one context never verifies under another when the challenge is
context-injective, an empty context is distinguishable from a real one, and the
binding cannot be stripped: this is why a valid attestation cannot be replayed
under a different capsule.
-/

namespace Nonos.Stark.ContextBinding

/-- A proof carries the data it committed to and the challenge the prover used. -/
structure Proof where
  data : Nat
  chal : Nat

/-- A verifier accepts when the proof's challenge is the one this context derives. -/
def valid (challenge : Nat → Nat → Nat) (ctx : Nat) (p : Proof) : Prop :=
  p.chal = challenge ctx p.data

/-- The challenge is context-injective: distinct contexts derive distinct challenges. -/
def CtxInjective (challenge : Nat → Nat → Nat) : Prop :=
  ∀ c₁ c₂ d, challenge c₁ d = challenge c₂ d → c₁ = c₂

/-- Verification is a function of the context and the proof: same inputs, same verdict. -/
theorem valid_deterministic (challenge : Nat → Nat → Nat) (ctx : Nat) (p : Proof)
    (h : p.chal = challenge ctx p.data) : valid challenge ctx p := h

/-- An honest prover under `ctx` produces a proof that verifies under `ctx`. -/
theorem honest_verifies (challenge : Nat → Nat → Nat) (ctx d : Nat) :
    valid challenge ctx ⟨d, challenge ctx d⟩ := rfl

/-- A proof valid under one context is not valid under a different one, when the
    challenge is context-injective: the attestation is bound to its identity. -/
theorem bound_to_context (challenge : Nat → Nat → Nat) (hc : CtxInjective challenge)
    (p : Proof) (ctx₁ ctx₂ : Nat) (hv : valid challenge ctx₁ p) (hne : ctx₁ ≠ ctx₂) :
    ¬ valid challenge ctx₂ p := by
  intro hv₂
  apply hne
  apply hc ctx₁ ctx₂ p.data
  rw [← hv, ← hv₂]

/-- Contrapositive: if a proof verifies under two contexts, they are equal. -/
theorem two_contexts_equal (challenge : Nat → Nat → Nat) (hc : CtxInjective challenge)
    (p : Proof) (ctx₁ ctx₂ : Nat) (h₁ : valid challenge ctx₁ p) (h₂ : valid challenge ctx₂ p) :
    ctx₁ = ctx₂ := by
  apply hc ctx₁ ctx₂ p.data
  rw [← h₁, ← h₂]

/-- A proof cannot be replayed under a different capsule identity. -/
theorem no_replay (challenge : Nat → Nat → Nat) (hc : CtxInjective challenge)
    (d : Nat) (ctx₁ ctx₂ : Nat) (hne : ctx₁ ≠ ctx₂) :
    ¬ valid challenge ctx₂ ⟨d, challenge ctx₁ d⟩ :=
  bound_to_context challenge hc _ ctx₁ ctx₂ rfl hne

/-- Only the challenge the context derives is accepted; any other is refused. -/
theorem only_bound_challenge (challenge : Nat → Nat → Nat) (ctx d c : Nat)
    (h : valid challenge ctx ⟨d, c⟩) : c = challenge ctx d := h

/-- Two proofs on the same data valid under the same context carry the same challenge. -/
theorem same_context_same_challenge (challenge : Nat → Nat → Nat) (ctx d c₁ c₂ : Nat)
    (h₁ : valid challenge ctx ⟨d, c₁⟩) (h₂ : valid challenge ctx ⟨d, c₂⟩) : c₁ = c₂ := by
  simp only [valid] at h₁ h₂; omega

/-- The empty context (0) is distinguishable from a real one under injectivity. -/
theorem empty_context_distinct (challenge : Nat → Nat → Nat) (hc : CtxInjective challenge)
    (ctx d : Nat) (hne : ctx ≠ 0) : challenge ctx d ≠ challenge 0 d := by
  intro h
  exact hne (hc ctx 0 d h)

/-- Stripping the context (verifying under 0) fails for a proof bound to a real one. -/
theorem cannot_strip_context (challenge : Nat → Nat → Nat) (hc : CtxInjective challenge)
    (d ctx : Nat) (hne : ctx ≠ 0) : ¬ valid challenge 0 ⟨d, challenge ctx d⟩ :=
  no_replay challenge hc d ctx 0 hne

end Nonos.Stark.ContextBinding
