/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Capability-token admission (`src/capabilities/token/validate.rs`). A token is
honoured only when three independent gates all pass: its signature verifies, it
has not expired, and it has not been revoked. The boolean `is_token_valid` and
the `validate_token_full` result path both encode this, the latter in a fixed
priority order so a caller gets a definite reason. The theorems below fix that a
valid token clears all three gates, that failing any one gate rejects the token,
and that the boolean and the result path agree, so the two entry points can
never disagree about whether a token is admissible.
-/

namespace Nonos.CapToken

/-- The three admission gates as booleans: whether the signature verifies,
    whether the token is unexpired, and whether it has been revoked. -/
structure Token where
  sigValid : Bool
  notExpired : Bool
  revoked : Bool

/-- `is_token_valid`: signature valid and not expired and not revoked. -/
def isValid (t : Token) : Bool := t.sigValid && t.notExpired && !t.revoked

/-- The result of `validate_token_full`, in the code's priority order. -/
inductive Verdict where
  | ok
  | badSig
  | expired
  | revoked
  deriving DecidableEq

/-- `validate_token_full`, branch for branch. -/
def validateFull (t : Token) : Verdict :=
  if t.sigValid = false then .badSig
  else if t.notExpired = false then .expired
  else if t.revoked = true then .revoked
  else .ok

/-- A valid token has a verifying signature. -/
theorem valid_sig (t : Token) (h : isValid t = true) : t.sigValid = true := by
  unfold isValid at h
  cases hs : t.sigValid with
  | true => rfl
  | false => rw [hs] at h; simp at h

/-- A valid token has not expired. -/
theorem valid_not_expired (t : Token) (h : isValid t = true) : t.notExpired = true := by
  unfold isValid at h
  cases he : t.notExpired with
  | true => rfl
  | false => rw [he] at h; simp at h

/-- A valid token has not been revoked. -/
theorem valid_not_revoked (t : Token) (h : isValid t = true) : t.revoked = false := by
  unfold isValid at h
  cases hr : t.revoked with
  | false => rfl
  | true => rw [hr] at h; simp at h

/-- A revoked token is never valid: revocation is final. -/
theorem revoked_invalid (t : Token) (h : t.revoked = true) : isValid t = false := by
  unfold isValid; rw [h]; simp

/-- An expired token is never valid. -/
theorem expired_invalid (t : Token) (h : t.notExpired = false) : isValid t = false := by
  unfold isValid; rw [h]; simp

/-- A token whose signature does not verify is never valid. -/
theorem bad_sig_invalid (t : Token) (h : t.sigValid = false) : isValid t = false := by
  unfold isValid; rw [h]; simp

/-- The boolean gate and the result path agree: `validate_token_full` returns ok
    exactly when `is_token_valid` is true, so the two entry points never
    disagree about a token's admissibility. -/
theorem full_ok_iff_valid (t : Token) : validateFull t = .ok ↔ isValid t = true := by
  unfold validateFull isValid
  by_cases hs : t.sigValid = false
  · rw [if_pos hs, hs]; simp
  · rw [if_neg hs]
    have hs' : t.sigValid = true := by
      cases h : t.sigValid with
      | true => rfl
      | false => exact absurd h hs
    by_cases he : t.notExpired = false
    · rw [if_pos he, hs', he]; simp
    · rw [if_neg he]
      have he' : t.notExpired = true := by
        cases h : t.notExpired with
        | true => rfl
        | false => exact absurd h he
      by_cases hr : t.revoked = true
      · rw [if_pos hr, hs', he', hr]; simp
      · have hr' : t.revoked = false := by
          cases h : t.revoked with
          | false => rfl
          | true => exact absurd h hr
        rw [if_neg hr, hs', he', hr']; simp

end Nonos.CapToken
