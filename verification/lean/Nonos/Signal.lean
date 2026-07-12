/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Signal masking. A process carries a pending set and a blocked mask, both as
predicates over signal numbers. A signal is deliverable when it is pending and
not blocked. The theorems below show raising a signal makes it pending, a
blocked signal is not deliverable yet is still retained (masking never drops a
signal), and unblocking a retained signal makes it deliverable, so a masked
signal is deferred and never lost.
-/

namespace Nonos.Signal

/-- A set of signal numbers as a membership predicate. -/
def SigSet := Nat → Bool

/-- Add a signal to a set. -/
def insert (s : SigSet) (sig : Nat) : SigSet := fun x => if x = sig then true else s x

/-- Remove a signal from a set. -/
def remove (s : SigSet) (sig : Nat) : SigSet := fun x => if x = sig then false else s x

/-- A process: signals raised but not yet taken, and signals currently blocked. -/
structure Proc where
  pending : SigSet
  mask : SigSet

/-- A signal is deliverable when pending and not blocked. -/
def deliverable (p : Proc) (sig : Nat) : Prop :=
  p.pending sig = true ∧ p.mask sig = false

/-- Raise a signal: mark it pending. -/
def raise (p : Proc) (sig : Nat) : Proc := ⟨insert p.pending sig, p.mask⟩

/-- Block a signal. -/
def block (p : Proc) (sig : Nat) : Proc := ⟨p.pending, insert p.mask sig⟩

/-- Unblock a signal. -/
def unblock (p : Proc) (sig : Nat) : Proc := ⟨p.pending, remove p.mask sig⟩

/-- Raising a signal makes it pending. -/
theorem raise_pends (p : Proc) (sig : Nat) : (raise p sig).pending sig = true := by
  simp only [raise, insert]; simp

/-- Raising one signal disturbs no other. -/
theorem raise_other (p : Proc) (sig other : Nat) (h : other ≠ sig) :
    (raise p sig).pending other = p.pending other := by
  simp only [raise, insert]; simp [h]

/-- A blocked signal is not deliverable. -/
theorem blocked_not_deliverable (p : Proc) (sig : Nat) : ¬ deliverable (block p sig) sig := by
  simp only [deliverable, block, insert]; simp

/-- Blocking never drops a pending signal: it stays retained. -/
theorem blocked_signal_still_pending (p : Proc) (sig : Nat) (h : p.pending sig = true) :
    (block p sig).pending sig = true := by
  simp only [block]; exact h

/-- Unblocking a retained signal makes it deliverable, so a masked signal is
    deferred rather than lost. -/
theorem unblock_delivers (p : Proc) (sig : Nat) (h : p.pending sig = true) :
    deliverable (unblock p sig) sig := by
  simp only [deliverable, unblock, remove]; simp [h]

end Nonos.Signal
