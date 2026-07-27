/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification-level proof for the wallet's hex-digit parser (`event/hex_digit`,
used by the recipient field and the private-key import). A parsed nibble is
always in range and only the three hex alphabets are accepted, so a stray key
can never inject a value outside 0..15 into an address or a secret.
-/

namespace Nonos.WalletHex

/-- Parse one ASCII code as a hex nibble. Mirrors `hex_digit`: digits 0-9,
    lower a-f, upper A-F; anything else is rejected. -/
def nibble (c : Nat) : Option Nat :=
  if 48 ≤ c ∧ c ≤ 57 then some (c - 48)
  else if 97 ≤ c ∧ c ≤ 102 then some (c - 87)
  else if 65 ≤ c ∧ c ≤ 70 then some (c - 55)
  else none

/-- Every accepted nibble is a genuine hex value, strictly below sixteen. -/
theorem nibble_lt_16 (c v : Nat) (h : nibble c = some v) : v < 16 := by
  unfold nibble at h
  split at h
  · injection h with h; omega
  · split at h
    · injection h with h; omega
    · split at h
      · injection h with h; omega
      · simp at h

/-- A non-hex code is rejected: no out-of-alphabet key slips through. -/
theorem non_hex_rejected (c : Nat)
    (h1 : ¬ (48 ≤ c ∧ c ≤ 57)) (h2 : ¬ (97 ≤ c ∧ c ≤ 102))
    (h3 : ¬ (65 ≤ c ∧ c ≤ 70)) : nibble c = none := by
  unfold nibble; simp [h1, h2, h3]

/-- The digits 0-9 map onto exactly 0-9. -/
theorem digit_value (c : Nat) (h1 : 48 ≤ c) (h2 : c ≤ 57) :
    nibble c = some (c - 48) := by
  unfold nibble; simp [h1, h2]

end Nonos.WalletHex
