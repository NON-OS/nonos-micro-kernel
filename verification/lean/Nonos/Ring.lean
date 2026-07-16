/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Bounded ring buffer. A ring holds a count of items against a capacity. Push
adds one only if there is room; pop removes one. The theorems below show the
count never exceeds the capacity, a full ring refuses a push, and the in-bounds
invariant survives any sequence of pushes and pops, so a DMA or IPC ring never
overruns its backing memory.

The kernel's input ring runs the index arithmetic from
`src/kernel_core/surface_registry/ring_math.rs` at its post and drain sites. The
`mechanism_proofs` crate includes that file and proves with Kani that a wrapped
index stays within the capacity and that a full ring is detected when the head
would reach the tail, so the addressing this model relies on is proven of the
code the ring runs.
-/

namespace Nonos.Ring

/-- A ring: the number of queued items and the capacity. -/
structure Ring where
  count : Nat
  cap : Nat

/-- The ring is in bounds when its count does not exceed its capacity. -/
def ok (r : Ring) : Prop := r.count ≤ r.cap

/-- Push an item if there is room, otherwise refuse. -/
def push (r : Ring) : Ring := if r.count < r.cap then ⟨r.count + 1, r.cap⟩ else r

/-- Pop an item (saturating at empty). -/
def pop (r : Ring) : Ring := ⟨r.count - 1, r.cap⟩

/-- Push keeps the ring in bounds. -/
theorem push_ok (r : Ring) (h : ok r) : ok (push r) := by
  simp only [ok] at h ⊢
  simp only [push]
  by_cases hc : r.count < r.cap
  · simp only [if_pos hc]; omega
  · simp only [if_neg hc]; exact h

/-- Pop keeps the ring in bounds. -/
theorem pop_ok (r : Ring) (h : ok r) : ok (pop r) := by
  simp only [ok] at h ⊢; simp only [pop]; omega

/-- A full ring refuses a push, leaving it untouched: no overrun. -/
theorem full_push_refused (r : Ring) (h : r.cap ≤ r.count) : push r = r := by
  simp only [push, if_neg (show ¬ r.count < r.cap by omega)]

/-- Pop of an empty ring stays empty (no underflow). -/
theorem pop_empty : pop ⟨0, 5⟩ = ⟨0, 5⟩ := by simp only [pop]

/-- An operation sequence. -/
inductive Op | push | pop

def step (r : Ring) : Op → Ring
  | Op.push => push r
  | Op.pop => pop r

/-- One operation preserves the in-bounds invariant. -/
theorem step_ok (r : Ring) (op : Op) (h : ok r) : ok (step r op) := by
  cases op with
  | push => exact push_ok r h
  | pop => exact pop_ok r h

def run (r : Ring) : List Op → Ring
  | [] => r
  | op :: rest => run (step r op) rest

/-- The in-bounds invariant survives any sequence of pushes and pops: the ring
    never overruns its capacity, whatever the traffic. -/
theorem run_ok (r : Ring) (ops : List Op) (h : ok r) : ok (run r ops) := by
  induction ops generalizing r with
  | nil => exact h
  | cons op rest ih => exact ih (step r op) (step_ok r op h)

/-- One operation never changes the capacity. -/
theorem step_cap (r : Ring) (op : Op) : (step r op).cap = r.cap := by
  cases op with
  | push =>
    simp only [step, push]
    by_cases hc : r.count < r.cap
    · simp [hc]
    · simp [hc]
  | pop => simp [step, pop]

/-- The capacity is invariant across any operation sequence. -/
theorem run_cap (r : Ring) (ops : List Op) : (run r ops).cap = r.cap := by
  induction ops generalizing r with
  | nil => rfl
  | cons op rest ih => simp only [run]; rw [ih (step r op), step_cap]

/-- Count after any run never exceeds the original capacity. -/
theorem run_count_le_cap (r : Ring) (ops : List Op) (h : ok r) :
    (run r ops).count ≤ r.cap := by
  have hok := run_ok r ops h
  simp only [ok] at hok
  rw [run_cap r ops] at hok
  exact hok

end Nonos.Ring
