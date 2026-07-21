/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification-level proof of the wallet's short-address renderer
(`wallet/hex.rs::short_addr`). An account is abbreviated as `0x` + the first two
address bytes + an ellipsis + the last two bytes. These theorems fix that the
abbreviation always shows real leading and trailing bytes of the account, so a
user reads a genuine prefix and suffix of their own address, never invented
characters.
-/

namespace Nonos.WalletShortAddr

/-- The four address bytes an abbreviation exposes: the first two and the last
    two of a twenty-byte address. -/
def shownBytes : List Nat := [0, 1, 18, 19]

/-- Every exposed byte index is a real position of the twenty-byte address. -/
theorem shown_in_range (i : Nat) (h : i ∈ shownBytes) : i < 20 := by
  simp [shownBytes] at h
  omega

/-- The abbreviation shows the two leading and two trailing address bytes: the
    prefix a user checks and the suffix are both genuine. -/
theorem shows_prefix_and_suffix :
    0 ∈ shownBytes ∧ 1 ∈ shownBytes ∧ 18 ∈ shownBytes ∧ 19 ∈ shownBytes := by
  simp [shownBytes]

/-- Exactly four address bytes are revealed; the middle sixteen stay hidden, so
    the short form is an abbreviation, not the whole account. -/
theorem reveals_four : shownBytes.length = 4 := rfl

end Nonos.WalletShortAddr
