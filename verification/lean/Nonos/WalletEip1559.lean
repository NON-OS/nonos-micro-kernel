/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification-level proof for the wallet's EIP-1559 fee sizing
(`ipc/eip1559_fees.rs`). The tip is clamped to a sane band and the fee cap is
always at least the tip plus headroom over the base fee, so a transfer can
neither underpay the tip nor set a cap below the tip it commits to paying.
-/

namespace Nonos.WalletEip1559

def gwei : Nat := 1000000000

/-- Priority tip: the base fee tenth, clamped into [1 gwei, 3 gwei]. -/
def priority (base : Nat) : Nat := max (1 * gwei) (min (3 * gwei) (base / 10))

/-- Fee cap: two base fees of headroom plus the tip. -/
def maxFee (base : Nat) : Nat := 2 * base + priority base

/-- The tip is never below one gwei: a transfer always offers a real tip. -/
theorem tip_at_least_one_gwei (base : Nat) : priority base ≥ 1 * gwei := by
  unfold priority; omega

/-- The tip is capped at three gwei: no runaway overpay on a spiking base fee. -/
theorem tip_at_most_three_gwei (base : Nat) : priority base ≤ 3 * gwei := by
  unfold priority; omega

/-- The fee cap always covers the tip: the wallet never commits to a cap below
    the priority fee it promises the validator. -/
theorem cap_covers_tip (base : Nat) : maxFee base ≥ priority base := by
  unfold maxFee; omega

/-- The fee cap always leaves at least two base fees of headroom, so a modest
    base-fee rise between signing and inclusion still lands the transfer. -/
theorem cap_covers_two_base (base : Nat) : maxFee base ≥ 2 * base := by
  unfold maxFee; omega

end Nonos.WalletEip1559
