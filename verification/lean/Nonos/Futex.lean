/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Futex wait queue. Waiters join the tail of a queue and are woken from the head,
so the queue is first in first out. The theorems below show a waiter that
enqueues is present in the queue (never lost), waking returns the head, the
oldest waiter is woken first, and each wait grows and each wake shrinks the
queue by exactly one, so a wakeup is neither dropped nor duplicated.
-/

namespace Nonos.Futex

/-- The wait queue as a list of thread ids, head first. -/
abbrev Queue := List Nat

/-- A thread waits: it joins the tail of the queue. -/
def wait (q : Queue) (tid : Nat) : Queue := q ++ [tid]

/-- Wake the longest-waiting thread from the head. -/
def wake : Queue → Option Nat × Queue
  | [] => (none, [])
  | t :: rest => (some t, rest)

/-- A waiter that enqueues is present in the queue: no lost waiter. -/
theorem waiter_enqueued (q : Queue) (tid : Nat) : tid ∈ wait q tid := by
  simp [wait]

/-- Waking an empty queue yields nothing. -/
theorem wake_empty_none : wake ([] : Queue) = (none, []) := rfl

/-- Waking returns the head thread. -/
theorem wake_returns_head (t : Nat) (rest : Queue) : (wake (t :: rest)).1 = some t := rfl

/-- The oldest waiter is woken first. -/
theorem fifo_first_out (a b : Nat) : (wake (wait (wait [] a) b)).1 = some a := by
  simp [wait, wake]

/-- Each wait grows the queue by exactly one. -/
theorem wait_grows (q : Queue) (tid : Nat) : (wait q tid).length = q.length + 1 := by
  simp [wait]

/-- Each wake shrinks the queue by exactly one. -/
theorem wake_shrinks (t : Nat) (rest : Queue) :
    (wake (t :: rest)).2.length + 1 = (t :: rest).length := by
  simp [wake]

end Nonos.Futex
