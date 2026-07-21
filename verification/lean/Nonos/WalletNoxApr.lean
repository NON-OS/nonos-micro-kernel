/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification-level proof for the wallet's staking APR reader. The capsule
derives APR in basis points from the on-chain emission rate over the total
staked amount (`nox/apr_bps.rs`). These theorems pin the properties the UI
relies on: no division by an empty pool, a still pool reads zero, and more
emission never lowers the rate. The abstract `apr` mirrors the Rust `apr_bps`
with Nat standing in for the checked u128 arithmetic.
-/

namespace Nonos.WalletNoxApr

/-- Seconds per year, as used by the reader. -/
def secondsPerYear : Nat := 31536000

/-- APR in basis points, or `none` when nothing is staked. Mirrors
    `apr_bps(emission, total)`: `emission * secondsPerYear * 10000 / total`. -/
def apr (emission total : Nat) : Option Nat :=
  if total = 0 then none
  else some (emission * secondsPerYear * 10000 / total)

/-- An empty pool never divides: the reader returns a dash, not a number. -/
theorem empty_pool_no_rate (emission : Nat) : apr emission 0 = none := rfl

/-- A non-empty pool always yields a concrete rate. -/
theorem staked_pool_has_rate (emission total : Nat) (h : total ≠ 0) :
    ∃ bps, apr emission total = some bps := by
  unfold apr
  simp [h]

/-- A pool with zero emission reads exactly zero, never a fabricated yield. -/
theorem no_emission_zero_rate (total : Nat) (h : total ≠ 0) :
    apr 0 total = some 0 := by
  unfold apr
  simp [h]

/-- Right division is monotone in the dividend. Core Lean 4 ships no direct
    lemma for this, so it is proven from `div_mul_le_self`: for a positive
    divisor via the `le_div_iff_mul_le` characterisation, and trivially for a
    zero divisor where both quotients are zero. -/
theorem div_le_div_right' (c : Nat) {a b : Nat} (h : a ≤ b) : a / c ≤ b / c := by
  cases c with
  | zero => simp
  | succ k =>
    rw [Nat.le_div_iff_mul_le (Nat.succ_pos k)]
    exact Nat.le_trans (Nat.div_mul_le_self a (k + 1)) h

/-- APR is monotone in the emission rate: raising emissions can only raise the
    reported rate for a fixed pool, so the figure tracks the chain. -/
theorem apr_monotone_in_emission (e1 e2 total : Nat) (he : e1 ≤ e2) :
    (e1 * secondsPerYear * 10000 / total) ≤ (e2 * secondsPerYear * 10000 / total) := by
  apply div_le_div_right'
  apply Nat.mul_le_mul_right
  apply Nat.mul_le_mul_right
  exact he

end Nonos.WalletNoxApr
