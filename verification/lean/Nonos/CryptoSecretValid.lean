/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification-level proof of the keyring's secp256k1 secret guard
(`capsule_keyring/store/eth_valid.rs`). A private key is accepted only when it
is a scalar in `[1, n)` for the curve order `n`: never zero, never at or above
the order. These theorems fix that a generated or imported key that falls
outside the group is rejected before it can ever be stored or sign.
-/

namespace Nonos.CryptoSecretValid

/-- The guard: a secret is valid iff it is a nonzero scalar below the order. -/
def valid (n s : Nat) : Bool := decide (0 < s ∧ s < n)

/-- The zero scalar is always rejected: it is not a usable private key. -/
theorem zero_rejected (n : Nat) : valid n 0 = false := by
  unfold valid; simp

/-- A scalar at or above the curve order is rejected: no reduction is silently
    applied that could collapse two keys onto one. -/
theorem overflow_rejected (n s : Nat) (h : n ≤ s) : valid n s = false := by
  unfold valid; simp; omega

/-- A scalar strictly inside `[1, n)` is accepted. -/
theorem in_range_accepted (n s : Nat) (h1 : 0 < s) (h2 : s < n) : valid n s = true := by
  unfold valid; simp [h1, h2]

/-- Acceptance is exactly membership of the open-below range: the guard admits
    no key outside the group and rejects none inside it. -/
theorem valid_iff (n s : Nat) : valid n s = true ↔ (0 < s ∧ s < n) := by
  unfold valid; simp

end Nonos.CryptoSecretValid
