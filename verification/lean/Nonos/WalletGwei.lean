/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification-level proof of the gwei conversion the wallet uses to show a gas
price (`paint_send_side.rs`). Wei are divided by 10^9 for display. These
theorems fix that the shown whole-gwei figure never overstates the fee and
recovers the wei value up to the sub-gwei remainder, so the gas price on screen
is a faithful floor of what the chain reported.
-/

namespace Nonos.WalletGwei

def weiPerGwei : Nat := 1000000000

/-- Whole gwei shown for a wei amount. -/
def gwei (wei : Nat) : Nat := wei / weiPerGwei

/-- The displayed gwei never overstates the fee: `gwei wei * 10^9 ≤ wei`. -/
theorem never_overstates (wei : Nat) : gwei wei * weiPerGwei ≤ wei := by
  unfold gwei weiPerGwei; omega

/-- The whole gwei plus the sub-gwei remainder recovers the exact wei amount. -/
theorem reconstructs (wei : Nat) : gwei wei * weiPerGwei + wei % weiPerGwei = wei := by
  unfold gwei weiPerGwei; omega

/-- The conversion is monotone: a higher wei fee never shows fewer gwei, so the
    displayed price tracks congestion in the right direction. -/
theorem monotone (a b : Nat) (h : a ≤ b) : gwei a ≤ gwei b := by
  unfold gwei
  rw [Nat.le_div_iff_mul_le (show 0 < weiPerGwei by unfold weiPerGwei; omega)]
  exact Nat.le_trans (Nat.div_mul_le_self a weiPerGwei) h

end Nonos.WalletGwei
