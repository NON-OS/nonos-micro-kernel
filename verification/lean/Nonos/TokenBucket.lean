/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Token bucket rate limiting. Tokens accumulate up to a burst ceiling and are
spent by admitted requests. The theorems below show a refill never carries the
bucket past its burst, a spend of an affordable request lowers the balance by
exactly its cost, an unaffordable request cannot drive the balance negative,
and the burst ceiling is an invariant, so the limiter never admits more than
the configured burst.
-/

namespace Nonos.TokenBucket

/-- A token bucket: current tokens and the burst ceiling. -/
structure Bucket where
  tokens : Nat
  burst : Nat

/-- The invariant: the balance never exceeds the burst ceiling. -/
def valid (b : Bucket) : Prop := b.tokens ≤ b.burst

/-- A request of cost `n` is admissible when the balance covers it. -/
def allows (b : Bucket) (n : Nat) : Prop := n ≤ b.tokens

/-- Add `n` tokens, saturating at the burst ceiling. -/
def refill (b : Bucket) (n : Nat) : Bucket := ⟨min b.burst (b.tokens + n), b.burst⟩

/-- Spend `n` tokens. -/
def take (b : Bucket) (n : Nat) : Bucket := ⟨b.tokens - n, b.burst⟩

/-- A refill never carries the balance past the burst ceiling. -/
theorem refill_never_exceeds_burst (b : Bucket) (n : Nat) : (refill b n).tokens ≤ b.burst := by
  simp only [refill]; omega

/-- A refill preserves the invariant. -/
theorem refill_valid (b : Bucket) (n : Nat) : valid (refill b n) := by
  simp only [valid, refill]; omega

/-- A spend preserves the invariant. -/
theorem take_valid (b : Bucket) (n : Nat) (h : valid b) : valid (take b n) := by
  simp only [valid, take] at *; omega

/-- An admitted request lowers the balance by exactly its cost. -/
theorem take_reduces (b : Bucket) (n : Nat) (h : allows b n) : (take b n).tokens + n = b.tokens := by
  simp only [take, allows] at *; omega

/-- An unaffordable request cannot leave a positive balance to overspend from:
    the balance floors at zero, never negative. -/
theorem cannot_overdraw (b : Bucket) (n : Nat) (h : ¬ allows b n) : (take b n).tokens = 0 := by
  simp only [allows, take] at *; omega

end Nonos.TokenBucket
