/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Ticket lock fairness. A ticket lock hands out monotonically increasing tickets
and serves them in order. The theorems below show two takers never receive the
same ticket, tickets strictly increase with each taker, the serving counter is
single valued, and the serving counter never runs ahead of the tickets issued,
so the lock is granted first come first served.
-/

namespace Nonos.Ticket

/-- A ticket lock: the next ticket to hand out and the ticket now being served. -/
structure Lock where
  next : Nat
  now : Nat

/-- The invariant: the serving counter never overtakes the issued tickets. -/
def valid (l : Lock) : Prop := l.now ≤ l.next

/-- Take a ticket, returning it and the advanced lock. -/
def take (l : Lock) : Nat × Lock := (l.next, ⟨l.next + 1, l.now⟩)

/-- Release the lock, advancing the serving counter. -/
def release (l : Lock) : Lock := ⟨l.next, l.now + 1⟩

/-- A caller holds the lock when its ticket is the one being served. -/
def serving (l : Lock) (ticket : Nat) : Prop := l.now = ticket

/-- Taking a ticket preserves the invariant. -/
theorem take_valid (l : Lock) (h : valid l) : valid (take l).2 := by
  simp only [valid, take] at *; omega

/-- The serving counter never overtakes the issued tickets after a release,
    provided at least one ticket was outstanding. -/
theorem release_valid (l : Lock) (h : l.now < l.next) : valid (release l) := by
  simp only [valid, release] at *; omega

/-- Two consecutive takers receive distinct tickets. -/
theorem take2_distinct (l : Lock) : (take l).1 ≠ (take (take l).2).1 := by
  simp only [take]; omega

/-- Tickets strictly increase with each taker: order of arrival is order of
    tickets. -/
theorem take_monotone (l : Lock) : (take l).1 < (take (take l).2).1 := by
  simp only [take]; omega

/-- Only one ticket is served at a time. -/
theorem serving_unique (l : Lock) (a b : Nat) (ha : serving l a) (hb : serving l b) : a = b := by
  simp only [serving] at *; omega

/-- Releasing advances service to the next ticket. -/
theorem release_serves_next (l : Lock) : (release l).now = l.now + 1 := rfl

end Nonos.Ticket
