/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification-level proof of EIP-2 low-s normalisation, used when the wallet
signs a transaction. An ECDSA signature `(r, s)` and `(r, n - s)` are both
valid, so a network that accepts high-s signatures is malleable. The signer
folds any high-s value into the low half of the order `n`. These theorems fix
that the normalised `s` is always in the canonical low range and that
normalising an already-low value changes nothing.
-/

namespace Nonos.CryptoLowS

/-- Low-s normalisation over curve order `n`: keep `s` if it is in the low half,
    else reflect it to `n - s`. Mirrors the `if s > n/2 { n - s } else { s }`
    the signer applies. -/
def normalize (n s : Nat) : Nat := if 2 * s ≤ n then s else n - s

/-- A normalised signature is always canonical: `s ≤ n/2`. This is the property
    that removes transaction malleability. -/
theorem normalized_is_low (n s : Nat) (h : s < n) : normalize n s ≤ n / 2 := by
  unfold normalize
  split <;> omega

/-- Normalising an already-low `s` is the identity: a canonical signature is
    left untouched. -/
theorem low_is_fixed (n s : Nat) (h : 2 * s ≤ n) : normalize n s = s := by
  unfold normalize; simp [h]

/-- Normalisation is idempotent: applying it twice equals applying it once, so
    the canonical form is stable. -/
theorem normalize_idempotent (n s : Nat) (h : s < n) :
    normalize n (normalize n s) = normalize n s := by
  have hlow : normalize n s ≤ n / 2 := normalized_is_low n s h
  have hfit : 2 * normalize n s ≤ n := by omega
  exact low_is_fixed n (normalize n s) hfit

/-- Reflection stays within the group: a normalised value never exceeds `n`. -/
theorem normalized_lt_n (n s : Nat) (h : s < n) : normalize n s < n := by
  unfold normalize
  split <;> omega

end Nonos.CryptoLowS
