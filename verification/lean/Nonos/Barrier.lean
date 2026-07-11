/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Thread barrier. A barrier releases its parties only once all of them have
arrived. The theorems below show the barrier is not released while fewer than
all parties have arrived, the final arrival releases it, arrivals count up
monotonically, and the arrival count never exceeds the party count, so no
thread passes the barrier early.
-/

namespace Nonos.Barrier

/-- A barrier: the number of parties required and how many have arrived. -/
structure Barrier where
  n : Nat
  arrived : Nat

/-- The invariant: never more arrivals than parties. -/
def valid (b : Barrier) : Prop := b.arrived ≤ b.n

/-- A thread reaches the barrier. -/
def arrive (b : Barrier) : Barrier := ⟨b.n, b.arrived + 1⟩

/-- The barrier is released once all parties have arrived. -/
def released (b : Barrier) : Prop := b.n ≤ b.arrived

/-- The barrier is not released while fewer than all parties have arrived. -/
theorem not_released_before_all (b : Barrier) (h : b.arrived < b.n) : ¬ released b := by
  simp only [released]; omega

/-- The final arrival releases the barrier. -/
theorem released_when_all_arrive (b : Barrier) (h : b.arrived + 1 = b.n) :
    released (arrive b) := by
  simp only [released, arrive]; omega

/-- Arrivals count up strictly. -/
theorem arrive_increases (b : Barrier) : b.arrived < (arrive b).arrived := by
  simp only [arrive]; omega

/-- An arrival preserves the invariant while parties remain outstanding. -/
theorem arrive_valid (b : Barrier) (h : b.arrived < b.n) : valid (arrive b) := by
  simp only [valid, arrive] at *; omega

end Nonos.Barrier
