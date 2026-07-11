/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Sequence lock consistency. A writer bumps a sequence counter on entry and exit,
leaving it odd while a write is in progress and even when stable. A reader
samples the counter before and after and accepts only if it was even and
unchanged. The theorems below show a write in progress makes the counter odd,
a matched write pair returns it to even, and a reader rejects both an
in-progress start and any value that changed under it, so a torn read is never
accepted.

The kernel implements this as a seqlock in `src/sys/sync/seqlock`, used for
read-mostly data. The sequence discipline in that module's `pure.rs` is the
same predicate set these theorems describe, and the `sync_proofs` crate runs it
against this model with differential tests and Kani, so the property is checked
of the code the kernel executes, not only of the model.
-/

namespace Nonos.Seqlock

/-- The sequence counter. -/
structure Seq where
  seq : Nat

/-- The counter is stable (no write in progress) when it is even. -/
def stable (s : Seq) : Prop := s.seq % 2 = 0

/-- A writer enters, making the counter odd. -/
def writeBegin (s : Seq) : Seq := ⟨s.seq + 1⟩

/-- A writer exits, making the counter even again. -/
def writeEnd (s : Seq) : Seq := ⟨s.seq + 1⟩

/-- A reader accepts a read iff the counter was even and unchanged across it. -/
def readAccepts (before after : Nat) : Prop := before = after ∧ before % 2 = 0

/-- A write in progress leaves the counter unstable. -/
theorem write_in_progress_odd (s : Seq) (h : stable s) : ¬ stable (writeBegin s) := by
  simp only [stable, writeBegin] at *; omega

/-- A matched write pair returns the counter to stable. -/
theorem write_pair_stable (s : Seq) (h : stable s) : stable (writeEnd (writeBegin s)) := by
  simp only [stable, writeBegin, writeEnd] at *; omega

/-- A reader that sampled an in-progress (odd) start rejects the read. -/
theorem odd_start_rejected (before after : Nat) (h : before % 2 = 1) :
    ¬ readAccepts before after := by
  intro hc; obtain ⟨_, he⟩ := hc; omega

/-- A reader whose samples differ rejects the read. -/
theorem changed_rejected (before after : Nat) (h : before ≠ after) :
    ¬ readAccepts before after := by
  intro hc; obtain ⟨he, _⟩ := hc; exact h he

/-- A stable, unchanged read is accepted. -/
theorem clean_read_accepts (v : Nat) (h : v % 2 = 0) : readAccepts v v := ⟨rfl, h⟩

end Nonos.Seqlock
