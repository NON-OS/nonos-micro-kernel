/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Reader-writer exclusion. A reader-writer lock admits many readers or one
writer, never both. The invariant is that a held write implies no readers. The
theorems below show that under the invariant a writer excludes all readers, a
read taken while no writer holds preserves the invariant, a write taken while
free and empty preserves it, and any reader present blocks a writer, so readers
and a writer never overlap.
-/

namespace Nonos.Rwlock

/-- A reader-writer lock: a reader count and a writer-held flag. -/
structure Rw where
  readers : Nat
  writer : Bool

/-- The invariant: a held write means no readers. -/
def valid (l : Rw) : Prop := l.writer = true → l.readers = 0

/-- A read may be taken when no writer holds. -/
def canRead (l : Rw) : Prop := l.writer = false

/-- A write may be taken when no reader and no writer holds. -/
def canWrite (l : Rw) : Prop := l.readers = 0 ∧ l.writer = false

/-- Take a read reference. -/
def acquireRead (l : Rw) : Rw := ⟨l.readers + 1, l.writer⟩

/-- Take the write reference. -/
def acquireWrite (l : Rw) : Rw := ⟨l.readers, true⟩

/-- Under the invariant, a writer excludes all readers. -/
theorem writer_excludes_readers (l : Rw) (h : valid l) (hw : l.writer = true) : l.readers = 0 :=
  h hw

/-- A read taken while no writer holds preserves the invariant. -/
theorem acquireRead_valid (l : Rw) (hc : canRead l) : valid (acquireRead l) := by
  simp only [valid, acquireRead, canRead] at *
  intro hw; simp [hc] at hw

/-- A write taken while free and empty preserves the invariant. -/
theorem acquireWrite_valid (l : Rw) (hc : canWrite l) : valid (acquireWrite l) := by
  simp only [valid, acquireWrite, canWrite] at *
  intro _; exact hc.1

/-- Any reader present blocks a writer. -/
theorem readers_block_write (l : Rw) (h : 0 < l.readers) : ¬ canWrite l := by
  simp only [canWrite]; intro hc; obtain ⟨h0, _⟩ := hc; omega

/-- Taking the write reference marks the writer held. -/
theorem acquireWrite_holds (l : Rw) : (acquireWrite l).writer = true := rfl

end Nonos.Rwlock
