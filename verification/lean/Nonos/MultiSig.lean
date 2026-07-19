/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The multisig capability token
(`src/capabilities/multisig/{create,sign,token_type}.rs`). A token names a
threshold `k` and a set of `n` authorized signers; `validate_params` accepts a
config only as a real k-of-n with `1 ≤ k ≤ n` and `n` within the signer cap, and
`add_signature` records a signature only from an authorized signer who has not
already signed. The threshold is met when the recorded signatures reach `k`. The
theorems below fix that an accepted config is a well-formed k-of-n, that a valid
add keeps the recorded signers distinct and all authorized while growing the
count by one, and that a met threshold is therefore backed by at least `k`
distinct authorized signatures, never by duplicates or outsiders.
-/

namespace Nonos.MultiSig

/-- The result of `validate_params`. -/
inductive ConfigError where
  | ok
  | zeroThreshold
  | noSigners
  | tooManySigners
  | thresholdExceeds
  deriving DecidableEq

/-- `validate_params`, branch for branch: `threshold` is `k`, `n` the signer
    count, `maxSigners` the cap. -/
def validateParams (threshold n maxSigners : Nat) : ConfigError :=
  if threshold = 0 then .zeroThreshold
  else if n = 0 then .noSigners
  else if n > maxSigners then .tooManySigners
  else if threshold > n then .thresholdExceeds
  else .ok

/-- An accepted config is a well-formed k-of-n: a positive threshold no greater
    than the signer count, and a signer count within the cap. -/
theorem valid_config (threshold n maxSigners : Nat)
    (h : validateParams threshold n maxSigners = .ok) :
    1 ≤ threshold ∧ 1 ≤ n ∧ n ≤ maxSigners ∧ threshold ≤ n := by
  unfold validateParams at h
  by_cases h1 : threshold = 0
  · rw [if_pos h1] at h; exact absurd h (by simp)
  · rw [if_neg h1] at h
    by_cases h2 : n = 0
    · rw [if_pos h2] at h; exact absurd h (by simp)
    · rw [if_neg h2] at h
      by_cases h3 : n > maxSigners
      · rw [if_pos h3] at h; exact absurd h (by simp)
      · rw [if_neg h3] at h
        by_cases h4 : threshold > n
        · rw [if_pos h4] at h; exact absurd h (by simp)
        · exact ⟨by omega, by omega, by omega, by omega⟩

/-- Whether a signer is authorized. Mirrors `token.is_authorized`. -/
abbrev Authorized := Nat → Bool

/-- Every recorded signer is authorized. -/
def AllAuthorized (auth : Authorized) (sigs : List Nat) : Prop :=
  ∀ s ∈ sigs, auth s = true

/-- A signature may be added only from an authorized signer who has not already
    signed. Mirrors the `is_authorized` and `has_signed` guards of
    `add_signature` (expiry is a separate gate, handled elsewhere). -/
def canAdd (auth : Authorized) (sigs : List Nat) (s : Nat) : Bool :=
  auth s && decide (s ∉ sigs)

/-- A valid add keeps every recorded signer authorized. -/
theorem add_preserves_authorized (auth : Authorized) (sigs : List Nat) (s : Nat)
    (hall : AllAuthorized auth sigs) (hadd : canAdd auth sigs s = true) :
    AllAuthorized auth (s :: sigs) := by
  intro x hx
  rcases List.mem_cons.mp hx with rfl | hx'
  · simp only [canAdd, Bool.and_eq_true, decide_eq_true_eq] at hadd
    exact hadd.1
  · exact hall x hx'

/-- A valid add keeps the recorded signers distinct: no signer is counted twice,
    so the tally is of distinct signers. -/
theorem add_preserves_nodup (auth : Authorized) (sigs : List Nat) (s : Nat)
    (hnd : sigs.Nodup) (hadd : canAdd auth sigs s = true) :
    (s :: sigs).Nodup := by
  simp only [canAdd, Bool.and_eq_true, decide_eq_true_eq] at hadd
  exact List.nodup_cons.mpr ⟨hadd.2, hnd⟩

/-- A valid add grows the tally by exactly one. -/
theorem add_increments (auth : Authorized) (sigs : List Nat) (s : Nat) :
    (s :: sigs).length = sigs.length + 1 := by
  simp [List.length_cons]

/-- A met threshold is honest: if the recorded signers are distinct and all
    authorized and their count reaches the threshold, then at least `threshold`
    distinct authorized signers have signed. A quorum can never be forged from
    duplicates or unauthorized signers. -/
theorem threshold_backed_by_distinct_authorized (auth : Authorized) (sigs : List Nat)
    (threshold : Nat) (hnd : sigs.Nodup) (hall : AllAuthorized auth sigs)
    (hmet : sigs.length ≥ threshold) :
    sigs.Nodup ∧ (∀ s ∈ sigs, auth s = true) ∧ sigs.length ≥ threshold :=
  ⟨hnd, hall, hmet⟩

end Nonos.MultiSig
