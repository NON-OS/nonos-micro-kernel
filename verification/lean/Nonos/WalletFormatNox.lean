/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification-level proof for the wallet's NOX amount formatter
(`nox/format_nox.rs`). An 18-decimal wei amount is shown as whole units and two
decimal places. These theorems fix that the split is faithful: the whole and
fractional parts reconstruct the exact wei value, and the two shown digits are
always a real hundredth, never rounded past 99.
-/

namespace Nonos.WalletFormatNox

def weiPerNox : Nat := 1000000000000000000
def weiPerCent : Nat := 10000000000000000

/-- Whole NOX units. -/
def whole (wei : Nat) : Nat := wei / weiPerNox
/-- Hundredths of a NOX shown after the point. -/
def cents (wei : Nat) : Nat := (wei % weiPerNox) / weiPerCent

/-- The two shown digits are a genuine hundredth: never 100 or more, so the
    decimal place is always a valid `00`..`99`. -/
theorem cents_lt_100 (wei : Nat) : cents wei < 100 := by
  unfold cents weiPerNox weiPerCent
  omega

/-- The whole part times 10^18 plus the wei remainder is exactly the input:
    the formatter loses nothing off the top of the balance. -/
theorem whole_reconstructs (wei : Nat) :
    whole wei * weiPerNox + wei % weiPerNox = wei := by
  unfold whole weiPerNox
  omega

/-- Zero wei shows as zero whole units and zero cents: an empty balance never
    prints a phantom amount. -/
theorem zero_shows_zero : whole 0 = 0 ∧ cents 0 = 0 := by
  unfold whole cents weiPerNox weiPerCent; omega

end Nonos.WalletFormatNox
