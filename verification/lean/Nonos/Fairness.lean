/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Scheduling fairness for the round-robin ready queue of `Nonos.Scheduler`.

The safety and security work proves what the system never does. This module
proves a progress property of the scheduler itself: under any number of
scheduling steps, the ready set is invariant. Every task that is ready stays
ready, no task appears from nowhere, and the number of ready tasks is preserved.
A round-robin step is a left rotation of the queue, so no task can be dropped or
buried indefinitely by the scheduler: the only way out of the ready queue is to
be dequeued, never to be silently lost. This is the liveness counterpart to the
safety invariants, discharged over the real queue operations.
-/

import Nonos.Scheduler

namespace Nonos.Fairness

open Nonos.Scheduler

/-- `n` round-robin scheduling steps: rotate the ready queue `n` times. -/
def rotateN : Nat → Queue → Queue
  | 0, q => q
  | k + 1, q => rotate (rotateN k q)

@[simp] theorem rotateN_zero (q : Queue) : rotateN 0 q = q := rfl

@[simp] theorem rotateN_succ (k : Nat) (q : Queue) :
    rotateN (k + 1) q = rotate (rotateN k q) := rfl

/-- No ready task is ever lost or invented: after any number of scheduling
    steps, a task is ready exactly when it was ready to begin with. -/
theorem rotateN_mem (k : Nat) (q : Queue) (x : Nat) : x ∈ rotateN k q ↔ x ∈ q := by
  induction k with
  | zero => rfl
  | succ k ih => rw [rotateN_succ, rotate_mem]; exact ih

/-- The number of ready tasks is preserved by scheduling: the round-robin step
    neither grows nor shrinks the ready queue. -/
theorem rotateN_length (k : Nat) (q : Queue) : (rotateN k q).length = q.length := by
  induction k with
  | zero => rfl
  | succ k ih => rw [rotateN_succ, rotate_length]; exact ih

/-- No starvation by loss: a task that is ready stays ready under every schedule,
    however long. The scheduler can never make an admitted task disappear from
    the ready queue; a task leaves only by being dequeued to run. -/
theorem no_starvation_by_loss (k : Nat) (q : Queue) (t : Nat) (h : t ∈ q) :
    t ∈ rotateN k q := (rotateN_mem k q t).mpr h

/-- A non-empty ready queue stays non-empty under scheduling: there is always a
    task available to run, so the scheduler never stalls a ready system. -/
theorem never_stalls (k : Nat) (q : Queue) (h : q ≠ []) : rotateN k q ≠ [] := by
  intro he
  apply h
  have : (rotateN k q).length = q.length := rotateN_length k q
  rw [he] at this
  exact List.length_eq_zero.mp this.symm

/-! ### Bounded-wait fairness

The invariants above say no ready task is ever lost. The stronger progress
property is that every ready task is served after a bounded number of steps: a
task at position `i` reaches the head after exactly `i` round-robin rotations,
and its position is less than the queue length, so its wait is bounded. No task
is starved: it is scheduled within one pass over the ready queue. -/

/-- Peeling a rotation from the front of the step count. -/
theorem rotateN_succ' (k : Nat) (q : Queue) :
    rotateN (k + 1) q = rotateN k (rotate q) := by
  induction k generalizing q with
  | zero => rfl
  | succ k ih => rw [rotateN_succ, ih, rotateN_succ]

/-- Rotating a queue by the length of a prefix brings the element just past that
    prefix to the head: `i` round-robin steps schedule the task at position `i`. -/
theorem rotateN_to_head (pre post : Queue) (t : Nat) :
    rotateN pre.length (pre ++ t :: post) = t :: (post ++ pre) := by
  induction pre generalizing post with
  | nil => simp
  | cons a pre' ih =>
    have hlen : (a :: pre').length = pre'.length + 1 := rfl
    rw [hlen, rotateN_succ']
    have hr : rotate ((a :: pre') ++ t :: post) = pre' ++ t :: (post ++ [a]) := by
      simp only [List.cons_append, rotate, List.append_assoc, List.cons_append,
        List.nil_append]
    rw [hr, ih (post ++ [a])]
    simp [List.append_assoc]

/-- Bounded-wait fairness: a ready task is scheduled to the head within fewer
    steps than the queue length. No task waits more than one pass over the ready
    queue, so none is starved. -/
theorem reaches_head (q : Queue) (t : Nat) (h : t ∈ q) :
    ∃ k, k < q.length ∧ (rotateN k q).head? = some t := by
  obtain ⟨pre, post, hq⟩ := List.append_of_mem h
  subst hq
  refine ⟨pre.length, ?_, ?_⟩
  · rw [List.length_append, List.length_cons]; omega
  · rw [rotateN_to_head]; rfl

end Nonos.Fairness
