/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Reference counting. A refcount is live while positive. The theorems below show
increment always leaves it live, decrement of a zero count cannot underflow
(saturating at zero, matching the kernel's checked decrement), a count of one
decrements to dead exactly, and increment/decrement round-trips, so an object
is freed exactly when its last reference drops.

The kernel's page table runs the checked decrement in
`src/memory/page_info/manager/refcount.rs`, which `decrement_ref_count`
delegates to. The `mechanism_proofs` crate includes that file and proves with
Kani that a decrement never underflows, so the no-underflow property is proven
of the code the page table runs.
-/

namespace Nonos.Refcount

/-- A reference count. -/
structure Rc where
  count : Nat

/-- An object is live while it has at least one reference. -/
def live (r : Rc) : Prop := 0 < r.count

/-- Take a reference. -/
def inc (r : Rc) : Rc := ⟨r.count + 1⟩

/-- Drop a reference (saturating at zero, the kernel refuses to underflow). -/
def dec (r : Rc) : Rc := ⟨r.count - 1⟩

theorem inc_count (r : Rc) : (inc r).count = r.count + 1 := by simp only [inc]

theorem dec_count (r : Rc) : (dec r).count = r.count - 1 := by simp only [dec]

/-- Taking a reference always leaves the object live. -/
theorem inc_live (r : Rc) : live (inc r) := by simp only [live, inc]; omega

/-- Decrement never underflows: a zero count stays at zero. -/
theorem dec_zero_stays_zero : (dec ⟨0⟩).count = 0 := by simp only [dec]

/-- Increment then decrement is the identity on the count. -/
theorem inc_dec_id (r : Rc) : (dec (inc r)).count = r.count := by
  simp only [dec, inc]; omega

/-- Dropping the last reference makes the object dead. -/
theorem dec_from_one_dead (r : Rc) (h : r.count = 1) : ¬ live (dec r) := by
  simp only [live, dec]; omega

/-- A dead object stays dead under further drops. -/
theorem dead_dec_stays_dead (r : Rc) (h : ¬ live r) : ¬ live (dec r) := by
  simp only [live] at *; simp only [dec]; omega

/-- Take `n` references. -/
def incN (r : Rc) : Nat → Rc
  | 0 => r
  | n + 1 => inc (incN r n)

/-- Taking `n` references raises the count by exactly `n`. -/
theorem incN_count (r : Rc) (n : Nat) : (incN r n).count = r.count + n := by
  induction n with
  | zero => simp [incN]
  | succ k ih => simp only [incN, inc]; omega

/-- Any object with references taken is live. -/
theorem incN_live (r : Rc) (n : Nat) (h : 0 < n) : live (incN r n) := by
  simp only [live, incN_count]
  omega

end Nonos.Refcount
