/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The MSI-X interrupt bind validator
(`src/hardware/broker/irq/validate.rs::validate_msix_request`). Before the broker
programs a device's MSI-X vectors for a capsule it clears the request against the
kernel-side device state in a fixed error-priority order: unknown flags, a
non-MSI-X request, a bad vector count, no device handle, no MSI-X capability, a
count past the device table, bad MSI-X BARs, a non-device IRQ source, and an
existing grant. The theorems below fix the guarantees an admitted bind carries:
the vector count is non-zero and within both the broker pool and the device's
MSI-X table, the device really advertises MSI-X with valid memory-mapped BARs,
the source is a device interrupt, and the pid does not already hold a grant, so a
bind is all-or-nothing per device per pid and never over-programs the table.
-/

namespace Nonos.IrqBind

/-- The verdict, one constructor per `IrqBindError` arm plus success. -/
inductive Verdict where
  | ok
  | unsupportedFlags
  | badVectorCount
  | noDeviceHandle
  | noMsixCap
  | badMsixBar
  | notDeviceIrq
  | alreadyBound
  deriving DecidableEq

/-- The request and device-state fields the validator reads. The bit tests
    `flags & !FLAGS_KNOWN == 0` and `flags & BIND_MSIX != 0` are kept as their
    boolean outcomes so the model stays in arithmetic. -/
structure Input where
  flagsSupported : Bool
  isMsix : Bool
  vectorCount : Nat
  poolCapacity : Nat
  handlePresent : Bool
  msixPresent : Bool
  msixTableSize : Nat
  barsOk : Bool
  irqSource : Nat
  hasExistingGrant : Bool

/-- `validate_msix_request`, branch for branch, in the code's error order. -/
def validate (i : Input) : Verdict :=
  if i.flagsSupported = false then .unsupportedFlags
  else if i.isMsix = false then .unsupportedFlags
  else if i.vectorCount = 0 ∨ i.vectorCount > i.poolCapacity then .badVectorCount
  else if i.handlePresent = false then .noDeviceHandle
  else if i.msixPresent = false then .noMsixCap
  else if i.vectorCount > i.msixTableSize then .badVectorCount
  else if i.barsOk = false then .badMsixBar
  else if i.irqSource ≠ 0 then .notDeviceIrq
  else if i.hasExistingGrant then .alreadyBound
  else .ok

/-- Characterization of an admitted bind: every guard passed. Every
    accepted-bind property is read off this lemma. -/
theorem accepted (i : Input) (h : validate i = .ok) :
    i.flagsSupported = true ∧ i.isMsix = true ∧
      i.vectorCount ≠ 0 ∧ i.vectorCount ≤ i.poolCapacity ∧
      i.handlePresent = true ∧ i.msixPresent = true ∧
      i.vectorCount ≤ i.msixTableSize ∧ i.barsOk = true ∧
      i.irqSource = 0 ∧ i.hasExistingGrant = false := by
  unfold validate at h
  by_cases h1 : i.flagsSupported = false
  · rw [if_pos h1] at h; exact absurd h (by simp)
  · rw [if_neg h1] at h
    by_cases h2 : i.isMsix = false
    · rw [if_pos h2] at h; exact absurd h (by simp)
    · rw [if_neg h2] at h
      by_cases h3 : i.vectorCount = 0 ∨ i.vectorCount > i.poolCapacity
      · rw [if_pos h3] at h; exact absurd h (by simp)
      · rw [if_neg h3] at h
        by_cases h4 : i.handlePresent = false
        · rw [if_pos h4] at h; exact absurd h (by simp)
        · rw [if_neg h4] at h
          by_cases h5 : i.msixPresent = false
          · rw [if_pos h5] at h; exact absurd h (by simp)
          · rw [if_neg h5] at h
            by_cases h6 : i.vectorCount > i.msixTableSize
            · rw [if_pos h6] at h; exact absurd h (by simp)
            · rw [if_neg h6] at h
              by_cases h7 : i.barsOk = false
              · rw [if_pos h7] at h; exact absurd h (by simp)
              · rw [if_neg h7] at h
                by_cases h8 : i.irqSource ≠ 0
                · rw [if_pos h8] at h; exact absurd h (by simp)
                · rw [if_neg h8] at h
                  by_cases h9 : i.hasExistingGrant = true
                  · rw [if_pos h9] at h; exact absurd h (by simp)
                  · refine ⟨?_, ?_, ?_, ?_, ?_, ?_, ?_, ?_, ?_, ?_⟩
                    · cases hb : i.flagsSupported <;> simp_all
                    · cases hb : i.isMsix <;> simp_all
                    · exact fun hz => h3 (Or.inl hz)
                    · exact Nat.le_of_not_lt (fun hgt => h3 (Or.inr hgt))
                    · cases hb : i.handlePresent <;> simp_all
                    · cases hb : i.msixPresent <;> simp_all
                    · exact Nat.le_of_not_lt h6
                    · cases hb : i.barsOk <;> simp_all
                    · exact Decidable.of_not_not h8
                    · cases hb : i.hasExistingGrant <;> simp_all

/-- The vector count of an admitted bind is within both the broker pool and the
    device's MSI-X table, so a bind never programs more vectors than either can
    hold. -/
theorem accepted_vector_count_bounded (i : Input) (h : validate i = .ok) :
    i.vectorCount ≠ 0 ∧ i.vectorCount ≤ i.poolCapacity ∧ i.vectorCount ≤ i.msixTableSize := by
  obtain ⟨_, _, hnz, hpool, _, _, htab, _⟩ := accepted i h
  exact ⟨hnz, hpool, htab⟩

/-- An admitted bind is on a device that really advertises MSI-X with valid,
    memory-mapped BARs: the broker never programs a table it cannot address. -/
theorem accepted_msix_addressable (i : Input) (h : validate i = .ok) :
    i.msixPresent = true ∧ i.barsOk = true :=
  ⟨(accepted i h).2.2.2.2.2.1, (accepted i h).2.2.2.2.2.2.2.1⟩

/-- An admitted bind does not overwrite an existing grant: MSI-X bind is
    all-or-nothing per device per pid. -/
theorem accepted_not_already_bound (i : Input) (h : validate i = .ok) :
    i.hasExistingGrant = false :=
  (accepted i h).2.2.2.2.2.2.2.2.2

/-- An admitted bind is for a device interrupt source, never a spoofed one. -/
theorem accepted_device_irq (i : Input) (h : validate i = .ok) : i.irqSource = 0 :=
  (accepted i h).2.2.2.2.2.2.2.2.1

/-- An INTx request routed through the MSI-X validator is refused. -/
theorem non_msix_rejected (i : Input) (h1 : i.flagsSupported = true) (h2 : i.isMsix = false) :
    validate i = .unsupportedFlags := by
  unfold validate; rw [if_neg (by simp [h1]), if_pos h2]

end Nonos.IrqBind
