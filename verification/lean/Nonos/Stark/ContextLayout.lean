/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Injectivity of the attestation context layout.

`ContextBinding` takes context-injectivity as a hypothesis. This discharges it
for the layout the kernel builds in `capsule_attest::stark::verify_against` and
`local_build::sign::context`: 48 bytes, measurement in [0,32), caps big-endian
in [32,40), epoch big-endian in [40,48).
-/

namespace Nonos.Stark.ContextLayout

/-- What the context commits to: the image, its capabilities, the policy epoch. -/
structure Ctx where
  measurement : Nat
  caps : Nat
  epoch : Nat

/-- Low field first: epoch, then caps scaled past it, then the measurement past
both. Widths stay abstract so the proof is about the layout, not the literals. -/
def pack (capsMod epochMod : Nat) (c : Ctx) : Nat :=
  c.epoch + epochMod * (c.caps + capsMod * c.measurement)

/-- Each field fits its width. -/
structure WellFormed (capsMod epochMod : Nat) (c : Ctx) : Prop where
  caps_lt : c.caps < capsMod
  epoch_lt : c.epoch < epochMod

/-- The epoch is the low field. -/
theorem epoch_of_pack (capsMod epochMod : Nat) (c : Ctx)
    (h : c.epoch < epochMod) :
    pack capsMod epochMod c % epochMod = c.epoch := by
  unfold pack
  rw [Nat.add_mul_mod_self_left]
  exact Nat.mod_eq_of_lt h

/-- Shifting past the epoch leaves caps and measurement. -/
theorem shift_epoch (capsMod epochMod : Nat) (c : Ctx)
    (hpos : 0 < epochMod) (h : c.epoch < epochMod) :
    pack capsMod epochMod c / epochMod = c.caps + capsMod * c.measurement := by
  unfold pack
  rw [Nat.add_mul_div_left _ _ hpos, Nat.div_eq_of_lt h, Nat.zero_add]

/-- Caps are recoverable. -/
theorem caps_of_pack (capsMod epochMod : Nat) (c : Ctx)
    (hpos : 0 < epochMod) (he : c.epoch < epochMod) (hc : c.caps < capsMod) :
    pack capsMod epochMod c / epochMod % capsMod = c.caps := by
  rw [shift_epoch capsMod epochMod c hpos he, Nat.add_mul_mod_self_left]
  exact Nat.mod_eq_of_lt hc

/-- The measurement is recoverable. -/
theorem measurement_of_pack (capsMod epochMod : Nat) (c : Ctx)
    (hep : 0 < epochMod) (hcp : 0 < capsMod)
    (he : c.epoch < epochMod) (hc : c.caps < capsMod) :
    pack capsMod epochMod c / epochMod / capsMod = c.measurement := by
  rw [shift_epoch capsMod epochMod c hep he, Nat.add_mul_div_left _ _ hcp,
      Nat.div_eq_of_lt hc, Nat.zero_add]

/-- The hypothesis `ContextBinding` assumes. Distinct triples give distinct
contexts, so a trailer minted for a capsule holding nothing does not verify for
the same bytes installed with the network bit. -/
theorem pack_injective (capsMod epochMod : Nat) (c₁ c₂ : Ctx)
    (hcp : 0 < capsMod) (hep : 0 < epochMod)
    (h₁ : WellFormed capsMod epochMod c₁) (h₂ : WellFormed capsMod epochMod c₂)
    (h : pack capsMod epochMod c₁ = pack capsMod epochMod c₂) : c₁ = c₂ := by
  have hE : c₁.epoch = c₂.epoch := by
    rw [← epoch_of_pack capsMod epochMod c₁ h₁.epoch_lt,
        ← epoch_of_pack capsMod epochMod c₂ h₂.epoch_lt, h]
  have hC : c₁.caps = c₂.caps := by
    rw [← caps_of_pack capsMod epochMod c₁ hep h₁.epoch_lt h₁.caps_lt,
        ← caps_of_pack capsMod epochMod c₂ hep h₂.epoch_lt h₂.caps_lt, h]
  have hM : c₁.measurement = c₂.measurement := by
    rw [← measurement_of_pack capsMod epochMod c₁ hep hcp h₁.epoch_lt h₁.caps_lt,
        ← measurement_of_pack capsMod epochMod c₂ hep hcp h₂.epoch_lt h₂.caps_lt, h]
  cases c₁
  cases c₂
  simp only [Ctx.mk.injEq]
  exact ⟨hM, hC, hE⟩

/-- Caps and epoch are eight bytes each. -/
def capsMod64 : Nat := 2 ^ 64
def epochMod64 : Nat := 2 ^ 64

theorem capsMod64_pos : 0 < capsMod64 := Nat.pos_pow_of_pos _ (by decide)
theorem epochMod64_pos : 0 < epochMod64 := Nat.pos_pow_of_pos _ (by decide)

/-- Injectivity at the kernel's widths. -/
theorem pack_injective_kernel (c₁ c₂ : Ctx)
    (h₁ : WellFormed capsMod64 epochMod64 c₁)
    (h₂ : WellFormed capsMod64 epochMod64 c₂)
    (h : pack capsMod64 epochMod64 c₁ = pack capsMod64 epochMod64 c₂) : c₁ = c₂ :=
  pack_injective capsMod64 epochMod64 c₁ c₂ capsMod64_pos epochMod64_pos h₁ h₂ h

end Nonos.Stark.ContextLayout
