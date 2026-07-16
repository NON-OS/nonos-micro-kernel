/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Monotonic time. The kernel clock only ever advances. The theorems below show a
tick never moves time backward, a positive tick moves it strictly forward, and
any sequence of ticks is monotone, so a timestamp read later is never earlier
than one read before, the property scheduling and anti-rollback deadlines rely
on.

The load balancer's elapsed-tick test in `src/process/scheduler/smp/interval.rs`,
which `should_balance` delegates to, is the same monotone comparison over a
saturating subtraction. The `mechanism_proofs` crate includes that file and
proves with Kani that it saturates on a tick wraparound, so the elapsed
computation is proven of the code the scheduler runs.
-/

namespace Nonos.Timer

/-- The monotonic clock: the current tick count. -/
structure Clock where
  now : Nat

/-- Advance the clock by `d`. -/
def tick (c : Clock) (d : Nat) : Clock := ⟨c.now + d⟩

theorem tick_now (c : Clock) (d : Nat) : (tick c d).now = c.now + d := by simp only [tick]

/-- A tick never moves time backward. -/
theorem tick_monotone (c : Clock) (d : Nat) : c.now ≤ (tick c d).now := by
  simp only [tick]; omega

/-- A positive tick moves time strictly forward. -/
theorem tick_strict (c : Clock) (d : Nat) (h : 0 < d) : c.now < (tick c d).now := by
  simp only [tick]; omega

/-- A zero tick leaves time unchanged. -/
theorem tick_zero (c : Clock) : (tick c 0).now = c.now := by simp [tick]

/-- Ticks compose additively. -/
theorem tick_add (c : Clock) (d e : Nat) :
    (tick (tick c d) e).now = c.now + d + e := by simp [tick]

/-- Advance the clock through a sequence of ticks. -/
def tickAll (c : Clock) : List Nat → Clock
  | [] => c
  | d :: rest => tickAll (tick c d) rest

/-- Any sequence of ticks is monotone: time never runs backward, whatever the
    schedule. -/
theorem tickAll_monotone (c : Clock) (ds : List Nat) : c.now ≤ (tickAll c ds).now := by
  induction ds generalizing c with
  | nil => exact Nat.le_refl _
  | cons d rest ih =>
    simp only [tickAll]
    exact Nat.le_trans (tick_monotone c d) (ih (tick c d))

end Nonos.Timer
