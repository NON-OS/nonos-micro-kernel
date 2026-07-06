/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification-level proof of the capability security model. The real kernel bit
operations that implement it (`has = bits & bit ≠ 0`, `grant = bits | bit`,
`revoke = bits & !bit`, attenuation) are proven to match by the Verus theorems
in `verification/verus/src/capabilities.rs`. Lean proves the abstract lattice
property; the Verus proof discharges it on the bit representation.
-/

namespace Nonos.Capability

/-- A capability set as its membership function over capability ids. -/
def Caps := Nat → Bool

/-- Holding capability `b`. -/
def Grants (c : Caps) (b : Nat) : Prop := c b = true

def empty : Caps := fun _ => false
def grant (c : Caps) (b : Nat) : Caps := fun x => c x || x == b
def revoke (c : Caps) (b : Nat) : Caps := fun x => c x && x != b
def attenuate (c mask : Caps) : Caps := fun x => c x && mask x

/-- The empty token grants nothing. -/
theorem empty_grants_nothing (b : Nat) : ¬ Grants empty b := by
  unfold Grants empty; simp

/-- Granting adds exactly the requested capability. -/
theorem grant_adds (c : Caps) (b : Nat) : Grants (grant c b) b := by
  unfold Grants grant; simp

/-- Granting never removes a capability already held (monotone). -/
theorem grant_never_removes (c : Caps) (b x : Nat) (h : Grants c x) :
    Grants (grant c b) x := by
  unfold Grants grant at *; simp [h]

/-- Revoking drops exactly the named capability. -/
theorem revoke_drops (c : Caps) (b : Nat) : ¬ Grants (revoke c b) b := by
  unfold Grants revoke; simp

/-- Revoking leaves every other capability untouched. -/
theorem revoke_preserves_others (c : Caps) (b x : Nat) (h : x ≠ b) :
    Grants (revoke c b) x ↔ Grants c x := by
  unfold Grants revoke; simp [h]

/-- Attenuation confines: an attenuated token never grants a capability the
    original token did not hold. -/
theorem attenuate_confines (c mask : Caps) (x : Nat)
    (h : Grants (attenuate c mask) x) : Grants c x := by
  unfold Grants attenuate at *; simp at h; exact h.1

/-- Attenuation is also confined by the mask: it grants nothing the mask lacks.
    Together with `attenuate_confines`, an attenuated token grants only what both
    the original token and the mask hold. -/
theorem attenuate_confined_by_mask (c mask : Caps) (x : Nat)
    (h : Grants (attenuate c mask) x) : Grants mask x := by
  unfold Grants attenuate at *; simp only [Bool.and_eq_true] at h; exact h.2

/-- Attenuating by the same mask twice is the same as attenuating once: there is
    no residual authority to strip on a second pass. -/
theorem attenuate_idempotent (c mask : Caps) (x : Nat) :
    Grants (attenuate (attenuate c mask) mask) x ↔ Grants (attenuate c mask) x := by
  unfold Grants attenuate; simp

/-- The order of two attenuations does not matter. -/
theorem attenuate_comm (c m1 m2 : Caps) (x : Nat) :
    Grants (attenuate (attenuate c m1) m2) x ↔ Grants (attenuate (attenuate c m2) m1) x := by
  unfold Grants attenuate
  simp only [Bool.and_eq_true]
  constructor
  · intro h; obtain ⟨⟨hc, h1⟩, h2⟩ := h; exact ⟨⟨hc, h2⟩, h1⟩
  · intro h; obtain ⟨⟨hc, h2⟩, h1⟩ := h; exact ⟨⟨hc, h1⟩, h2⟩

/-- Revoking a capability twice equals revoking it once. -/
theorem revoke_idempotent (c : Caps) (b x : Nat) :
    Grants (revoke (revoke c b) b) x ↔ Grants (revoke c b) x := by
  unfold Grants revoke; simp

/-- A grant cannot survive a following revoke of the same capability: revoking
    after granting is the same as revoking directly. -/
theorem grant_then_revoke_drops (c : Caps) (b : Nat) :
    ¬ Grants (revoke (grant c b) b) b := by
  unfold Grants revoke; simp

/-!
### The capability lattice

`Caps` under the delegation order below is a bounded meet-semilattice: `empty`
is the bottom, `attenuate` is the meet, and `grant` joins a token with a single
capability. The confinement legs of these theorems are discharged on the
kernel's bit operations by `verification/verus/src/capabilities.rs`
(`attenuation_confines`, `grant_preserves_and_adds`, `revoke_is_monotonic`,
`empty_token_grants_nothing`); the bound-optimality legs are model-side
algebra that pins down what those operations mean as an order.
-/

/-- The delegation order: `Leq c d` when every capability `c` grants, `d`
    grants. A child token sits below everything it was carved from. -/
def Leq (c d : Caps) : Prop := ∀ x, Grants c x → Grants d x

theorem leq_refl (c : Caps) : Leq c c := fun _ h => h

theorem leq_trans (a b c : Caps) (hab : Leq a b) (hbc : Leq b c) : Leq a c :=
  fun x h => hbc x (hab x h)

/-- `empty` is the bottom of the order: every token sits above it. -/
theorem empty_is_bottom (c : Caps) : Leq empty c := by
  intro x h
  exact absurd h (empty_grants_nothing x)

/-- Revocation only ever moves a token down the order. -/
theorem revoke_leq (c : Caps) (b : Nat) : Leq (revoke c b) c := by
  intro x h
  unfold Grants revoke at h
  simp only [Bool.and_eq_true] at h
  exact h.1

/-- Attenuation sits below the token it narrows. -/
theorem attenuate_leq_left (c mask : Caps) : Leq (attenuate c mask) c :=
  fun x h => attenuate_confines c mask x h

/-- Attenuation sits below its mask. -/
theorem attenuate_leq_right (c mask : Caps) : Leq (attenuate c mask) mask :=
  fun x h => attenuate_confined_by_mask c mask x h

/-- Attenuation is the greatest lower bound: any token below both the original
    and the mask is below the attenuation. So the kernel's `bits & mask` is not
    merely a restriction, it is the least restrictive one that respects both
    bounds; a delegate loses nothing that both sides allow. -/
theorem attenuate_is_glb (c mask d : Caps)
    (hc : Leq d c) (hm : Leq d mask) : Leq d (attenuate c mask) := by
  intro x h
  have h1 := hc x h
  have h2 := hm x h
  unfold Grants attenuate at *
  simp [h1, h2]

/-- Nested attenuations collapse to one intersection: attenuating by `m1` and
    then `m2` is attenuating by their meet. A delegation chain of any depth is
    a single mask. -/
theorem attenuate_assoc (c m1 m2 : Caps) (x : Nat) :
    Grants (attenuate (attenuate c m1) m2) x ↔
    Grants (attenuate c (attenuate m1 m2)) x := by
  unfold Grants attenuate
  simp [Bool.and_assoc]

/-- Attenuating a token by itself changes nothing: the meet is idempotent. -/
theorem attenuate_self (c : Caps) (x : Nat) :
    Grants (attenuate c c) x ↔ Grants c x := by
  unfold Grants attenuate
  simp

/-- Attenuating by the empty mask yields a token that grants nothing: the
    bottom absorbs the meet. -/
theorem attenuate_by_empty_grants_nothing (c : Caps) (x : Nat) :
    ¬ Grants (attenuate c empty) x := by
  unfold Grants attenuate empty
  simp

/-- The token holding exactly capability `b`. -/
def only (b : Nat) : Caps := fun x => x == b

/-- A token sits below itself with anything granted. -/
theorem leq_grant (c : Caps) (b : Nat) : Leq c (grant c b) := by
  intro x h
  unfold Grants grant at *
  simp [h]

/-- The granted capability alone sits below the grant. -/
theorem only_leq_grant (c : Caps) (b : Nat) : Leq (only b) (grant c b) := by
  intro x h
  unfold Grants grant only at *
  simp [h]

/-- Granting is the least upper bound of the token and the single capability:
    anything above both is above the grant. So the kernel's `bits | bit` adds
    the one capability and nothing else; no unrelated authority rides along. -/
theorem grant_is_lub (c d : Caps) (b : Nat)
    (hc : Leq c d) (hb : Leq (only b) d) : Leq (grant c b) d := by
  intro x h
  unfold Grants grant at h
  simp only [Bool.or_eq_true] at h
  obtain hcx | hxb := h
  · exact hc x hcx
  · exact hb x hxb

/-- A delegation chain: each holder attenuates by its own mask and passes the
    result on. -/
def attenuateAll (c : Caps) : List Caps → Caps
  | [] => c
  | m :: ms => attenuateAll (attenuate c m) ms

/-- No chain of delegations, of any length, widens authority: whatever the
    final token grants, the original token already granted. This lifts
    `attenuate_confines` from one step to every execution. -/
theorem chain_never_widens (c : Caps) (ms : List Caps) (x : Nat)
    (h : Grants (attenuateAll c ms) x) : Grants c x := by
  induction ms generalizing c with
  | nil => exact h
  | cons m ms ih => exact attenuate_confines c m x (ih (attenuate c m) h)

end Nonos.Capability
