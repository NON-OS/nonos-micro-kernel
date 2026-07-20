/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Capability binding in the attestation context. The gate binds the image hash, the
granted capability set, and the policy epoch into one context before proving, so an
attestation is valid only for the exact capabilities it was granted under. The
theorems below show the context is a function of all three inputs and, under an
injective binder, changing the image, the capabilities, or the epoch changes the
context: this is why a capsule cannot carry a proof issued for a weaker capability
set into a stronger one.
-/

namespace Nonos.Stark.CapabilityBinding

/-- The proving context: image hash, granted capabilities, policy epoch. -/
structure Context where
  image : Nat
  caps : Nat
  epoch : Nat

/-- Bind a context into a single field value through an abstract binder. -/
def bind (mk : Nat → Nat → Nat → Nat) (c : Context) : Nat := mk c.image c.caps c.epoch

/-- The binder is injective in all three coordinates. -/
def BindInjective (mk : Nat → Nat → Nat → Nat) : Prop :=
  ∀ a b c a' b' c', mk a b c = mk a' b' c' → a = a' ∧ b = b' ∧ c = c'

/-- Binding is a function of the context. -/
theorem bind_deterministic (mk : Nat → Nat → Nat → Nat) (c : Context) (x y : Nat)
    (h₁ : bind mk c = x) (h₂ : bind mk c = y) : x = y := by rw [← h₁, ← h₂]

/-- Under an injective binder, equal bound contexts are equal contexts. -/
theorem bind_injective (mk : Nat → Nat → Nat → Nat) (hb : BindInjective mk)
    (c₁ c₂ : Context) (h : bind mk c₁ = bind mk c₂) : c₁ = c₂ := by
  obtain ⟨hi, hc, he⟩ := hb _ _ _ _ _ _ h
  cases c₁; cases c₂; simp_all

/-- Changing the granted capabilities changes the context. -/
theorem different_caps_different_context (mk : Nat → Nat → Nat → Nat) (hb : BindInjective mk)
    (img caps caps' epoch : Nat) (hne : caps ≠ caps') :
    bind mk ⟨img, caps, epoch⟩ ≠ bind mk ⟨img, caps', epoch⟩ := by
  intro h
  exact hne (hb _ _ _ _ _ _ h).2.1

/-- Changing the image changes the context. -/
theorem different_image_different_context (mk : Nat → Nat → Nat → Nat) (hb : BindInjective mk)
    (img img' caps epoch : Nat) (hne : img ≠ img') :
    bind mk ⟨img, caps, epoch⟩ ≠ bind mk ⟨img', caps, epoch⟩ := by
  intro h
  exact hne (hb _ _ _ _ _ _ h).1

/-- Changing the policy epoch changes the context. -/
theorem different_epoch_different_context (mk : Nat → Nat → Nat → Nat) (hb : BindInjective mk)
    (img caps epoch epoch' : Nat) (hne : epoch ≠ epoch') :
    bind mk ⟨img, caps, epoch⟩ ≠ bind mk ⟨img, caps, epoch'⟩ := by
  intro h
  exact hne (hb _ _ _ _ _ _ h).2.2

/-- A proof issued for one capability set does not verify under another. -/
theorem cap_escalation_refused (mk : Nat → Nat → Nat → Nat) (hb : BindInjective mk)
    (img granted requested epoch : Nat) (hne : granted ≠ requested)
    (proofCtx : Nat) (hproof : proofCtx = bind mk ⟨img, granted, epoch⟩) :
    proofCtx ≠ bind mk ⟨img, requested, epoch⟩ := by
  rw [hproof]
  exact different_caps_different_context mk hb img granted requested epoch hne

end Nonos.Stark.CapabilityBinding
