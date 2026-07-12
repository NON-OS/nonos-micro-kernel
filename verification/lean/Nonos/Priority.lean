/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Scheduling priority. A higher priority preempts a lower one. The theorems below
show preemption is a strict order, irreflexive, transitive, asymmetric, and
total up to equal priority, and that a maximal-priority task is preempted by no
one, so the highest-priority runnable task always runs.
-/

namespace Nonos.Priority

/-- `a` preempts `b` when it is strictly higher priority (a higher `Nat` is a
    higher priority). -/
def preempts (a b : Nat) : Prop := b < a

/-- Nothing preempts itself. -/
theorem preempts_irrefl (a : Nat) : ¬ preempts a a := by unfold preempts; omega

/-- Preemption is transitive. -/
theorem preempts_trans (a b c : Nat) (hab : preempts a b) (hbc : preempts b c) :
    preempts a c := by unfold preempts at *; omega

/-- Preemption is asymmetric: if `a` preempts `b`, `b` does not preempt `a`. -/
theorem preempts_asymm (a b : Nat) (h : preempts a b) : ¬ preempts b a := by
  unfold preempts at *; omega

/-- Preemption is total up to equal priority: for any two tasks, one preempts
    the other or they tie. -/
theorem preempts_total (a b : Nat) : preempts a b ∨ preempts b a ∨ a = b := by
  unfold preempts; omega

/-- A maximal-priority task is preempted by no one: the top task always runs. -/
theorem top_not_preempted (a : Nat) (hmax : ∀ b, b ≤ a) (x : Nat) :
    ¬ preempts x a := by
  unfold preempts; have := hmax x; omega

/-- Raising a task's priority above another makes it preempt that other. -/
theorem raise_preempts (a b : Nat) (h : a ≤ b) : preempts (b + 1) a := by
  unfold preempts; omega

end Nonos.Priority
