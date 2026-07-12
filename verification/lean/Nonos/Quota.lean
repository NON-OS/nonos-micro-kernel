/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Resource quotas. A quota tracks used units against a cap. Acquire succeeds only
if it keeps use within the cap; release lowers use. The theorems below show the
in-bounds invariant survives any single operation and any whole sequence of
acquires, a capsule can never be charged past its quota.

The kernel's resource token gates a charge with the check in
`src/capabilities/resource/limits.rs`, which `has_bytes` and `has_ops` delegate
to. The `mechanism_proofs` crate includes that file and proves with Kani that a
request is admitted exactly when it is within the remaining budget, so the gate
this model relies on is proven of the code the token runs.
-/

namespace Nonos.Quota

/-- A quota: units currently used, and the cap. -/
structure Q where
  used : Nat
  cap : Nat

/-- The quota is in bounds when use does not exceed the cap. -/
def ok (q : Q) : Prop := q.used ≤ q.cap

/-- Charge `n` units if it stays within the cap, otherwise refuse. -/
def acquire (q : Q) (n : Nat) : Q :=
  if q.used + n ≤ q.cap then { q with used := q.used + n } else q

/-- Return `n` units. -/
def release (q : Q) (n : Nat) : Q := { q with used := q.used - n }

/-- Acquire keeps the quota in bounds. -/
theorem acquire_preserves_ok (q : Q) (n : Nat) (h : ok q) : ok (acquire q n) := by
  unfold ok at h ⊢
  simp only [acquire]
  by_cases hc : q.used + n ≤ q.cap
  · rw [if_pos hc]; exact hc
  · rw [if_neg hc]; exact h

/-- Release keeps the quota in bounds. -/
theorem release_preserves_ok (q : Q) (n : Nat) (h : ok q) : ok (release q n) := by
  unfold ok at h ⊢
  simp only [release]
  omega

/-- A refused acquire leaves the quota untouched, so it can never push use over
    the cap. -/
theorem oversized_acquire_refused (q : Q) (n : Nat) (h : q.cap < q.used + n) :
    acquire q n = q := by
  simp only [acquire, if_neg (show ¬ q.used + n ≤ q.cap by omega)]

/-- A whole sequence of acquires. -/
def acquireAll (q : Q) : List Nat → Q
  | [] => q
  | n :: rest => acquireAll (acquire q n) rest

/-- The in-bounds invariant survives any sequence of acquires: a capsule is
    never charged past its quota, whatever it requests. -/
theorem acquireAll_ok (q : Q) (ns : List Nat) (h : ok q) : ok (acquireAll q ns) := by
  induction ns generalizing q with
  | nil => exact h
  | cons n rest ih => exact ih (acquire q n) (acquire_preserves_ok q n h)

/-- Acquire never changes the cap (it only ever touches `used`). -/
theorem acquire_cap (q : Q) (n : Nat) : (acquire q n).cap = q.cap := by
  simp only [acquire]
  by_cases hc : q.used + n ≤ q.cap
  · simp [if_pos hc]
  · simp [if_neg hc]

/-- The cap is invariant across any acquire sequence. -/
theorem acquireAll_cap (q : Q) (ns : List Nat) : (acquireAll q ns).cap = q.cap := by
  induction ns generalizing q with
  | nil => rfl
  | cons n rest ih => simp only [acquireAll]; rw [ih (acquire q n), acquire_cap]

/-- Use after any acquire sequence never exceeds the original cap. -/
theorem acquireAll_used_le_cap (q : Q) (ns : List Nat) (h : ok q) :
    (acquireAll q ns).used ≤ q.cap := by
  have hok := acquireAll_ok q ns h
  unfold ok at hok
  rw [acquireAll_cap q ns] at hok
  exact hok

end Nonos.Quota
