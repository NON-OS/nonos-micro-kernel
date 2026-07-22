/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification-level proof for the wallet's APR formatter (`nox/format_apr.rs`).
Basis points are shown as a percentage with two decimals. These theorems fix
that the integer and fractional parts reconstruct the exact bps figure and that
the two decimals are always a real hundredth of a percent.
-/

namespace Nonos.WalletFormatApr

/-- Whole percent from basis points. -/
def percent (bps : Nat) : Nat := bps / 100
/-- The two decimal digits (hundredths of a percent). -/
def frac (bps : Nat) : Nat := bps % 100

/-- The two decimals are always a genuine hundredth: below 100. -/
theorem frac_lt_100 (bps : Nat) : frac bps < 100 := by
  unfold frac; omega

/-- Percent times 100 plus the fractional digits is exactly the bps input: the
    displayed rate equals the on-chain figure, no rounding drift. -/
theorem reconstructs (bps : Nat) : percent bps * 100 + frac bps = bps := by
  unfold percent frac; omega

/-- 1290 bps renders as 12.90 percent, the live-mainnet figure this reader was
    checked against. -/
theorem live_vector : percent 1290 = 12 ∧ frac 1290 = 90 := by
  unfold percent frac; omega

end Nonos.WalletFormatApr
