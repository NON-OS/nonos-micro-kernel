/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Entropy pool progression. Each draw evolves the pool state strictly forward.
The theorems below show a draw advances the state, two draws never leave the
pool in the same state, and any number of draws only ever moves the state
forward, so the generator never stalls on a fixed state.
-/

namespace Nonos.Rng

/-- The entropy pool state. -/
structure Pool where
  state : Nat

/-- Draw a value and mix fresh input into the pool; the state strictly
    advances. -/
def draw (p : Pool) (mix : Nat) : Nat × Pool := (p.state, ⟨p.state + mix + 1⟩)

/-- A draw strictly advances the pool state. -/
theorem draw_advances (p : Pool) (mix : Nat) : p.state < (draw p mix).2.state := by
  simp only [draw]; omega

/-- A draw never leaves the pool in the same state: no fixed point. -/
theorem draw_changes_state (p : Pool) (mix : Nat) : p.state ≠ (draw p mix).2.state := by
  simp only [draw]; omega

/-- The drawn value is the old state; the pool moves on past it. -/
theorem draw_value (p : Pool) (mix : Nat) : (draw p mix).1 = p.state := by simp only [draw]

/-- Draw `n` times. -/
def drawN (p : Pool) (mix : Nat) : Nat → Pool
  | 0 => p
  | n + 1 => drawN (draw p mix).2 mix n

/-- Any number of draws only moves the state forward: the pool never regresses
    to an earlier state. -/
theorem drawN_advances (p : Pool) (mix : Nat) (n : Nat) :
    p.state ≤ (drawN p mix n).state := by
  induction n generalizing p with
  | zero => simp [drawN]
  | succ k ih =>
    simp only [drawN]
    have h1 := ih (draw p mix).2
    have h2 := draw_advances p mix
    omega

/-- After at least one draw the state has strictly advanced from the start. -/
theorem drawN_strict (p : Pool) (mix : Nat) (n : Nat) (h : 0 < n) :
    p.state < (drawN p mix n).state := by
  cases n with
  | zero => omega
  | succ k =>
    simp only [drawN]
    have h1 := drawN_advances (draw p mix).2 mix k
    have h2 := draw_advances p mix
    omega

end Nonos.Rng
