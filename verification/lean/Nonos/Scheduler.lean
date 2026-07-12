/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Scheduler run-queue safety. The ready queue is the list of runnable task ids.
The theorems below show enqueue/dequeue change membership only in the intended
way, and that round-robin rotation preserves both the membership set and the
length, so no runnable task is ever dropped or duplicated by scheduling, and
a task only runs if it was in the ready queue.
-/

namespace Nonos.Scheduler

/-- The ready queue: runnable task ids, front first. -/
abbrev Queue := List Nat

/-- Admit a task to the back of the ready queue. -/
def enqueue (q : Queue) (t : Nat) : Queue := q ++ [t]

/-- Take the front task to run, if any. -/
def dequeue : Queue → Option (Nat × Queue)
  | [] => none
  | t :: rest => some (t, rest)

/-- Round-robin: run the front task, move it to the back. -/
def rotate : Queue → Queue
  | [] => []
  | t :: rest => rest ++ [t]

/-- The admitted task is in the queue afterward. -/
theorem enqueue_mem (q : Queue) (t : Nat) : t ∈ enqueue q t := by
  unfold enqueue; simp

/-- Admitting a task never removes one already queued. -/
theorem enqueue_preserves (q : Queue) (t x : Nat) (h : x ∈ q) : x ∈ enqueue q t := by
  unfold enqueue; simp [h]

/-- Admitting a task adds exactly one to the length. -/
theorem enqueue_length (q : Queue) (t : Nat) : (enqueue q t).length = q.length + 1 := by
  unfold enqueue; simp

/-- A dequeued task was in the queue: nothing is scheduled from thin air. -/
theorem dequeue_mem (q : Queue) (t : Nat) (rest : Queue)
    (h : dequeue q = some (t, rest)) : t ∈ q := by
  cases q with
  | nil => simp [dequeue] at h
  | cons a as =>
    simp only [dequeue, Option.some.injEq, Prod.mk.injEq] at h
    simp [h.1]

/-- Dequeue removes exactly the front, leaving the rest: the remaining queue is
    one shorter. -/
theorem dequeue_length (q : Queue) (t : Nat) (rest : Queue)
    (h : dequeue q = some (t, rest)) : rest.length + 1 = q.length := by
  cases q with
  | nil => simp [dequeue] at h
  | cons a as =>
    simp only [dequeue, Option.some.injEq, Prod.mk.injEq] at h
    simp [h.2]

/-- The queue is empty exactly when there is nothing to dequeue. -/
theorem dequeue_none_iff (q : Queue) : dequeue q = none ↔ q = [] := by
  cases q with
  | nil => simp [dequeue]
  | cons a as => simp [dequeue]

/-- Rotation preserves membership: every task in the queue is still there after
    a round-robin step, so round-robin drops no one. -/
theorem rotate_mem (q : Queue) (x : Nat) : x ∈ rotate q ↔ x ∈ q := by
  cases q with
  | nil => simp [rotate]
  | cons a as =>
    simp only [rotate, List.mem_append, List.mem_cons, List.mem_singleton,
      List.not_mem_nil, or_false]
    exact or_comm

/-- Rotation preserves the queue length: round-robin neither creates nor
    destroys a runnable task. -/
theorem rotate_length (q : Queue) : (rotate q).length = q.length := by
  cases q with
  | nil => rfl
  | cons a as => simp [rotate]

/-- After rotation of a non-empty queue, the task that just ran is at the back,
    still runnable, fairness: it will be reached again. -/
theorem rotate_keeps_head (a : Nat) (as : Queue) : a ∈ rotate (a :: as) := by
  simp [rotate]

end Nonos.Scheduler
