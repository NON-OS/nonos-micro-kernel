/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The ELF program-header table bounds check
(`src/elf/loader/core/parse_header/bounds.rs::program_header_bounds`). Before the
loader reads a single program header it proves the whole table lies inside the
image: an empty table is accepted trivially, a wrong entry size is refused, the
table byte length and its end offset are formed with checked arithmetic so a
crafted count or offset cannot wrap, and a table that runs past the image end is
refused. The theorems below fix that: an accepted non-empty table has the exact
entry size, its span never overflows the address type, and it ends at or before
the image end, so every header the loader then reads is in bounds. This is the
spatial-safety gate the whole segment loader sits behind.
-/

namespace Nonos.ElfPhdr

/-- The checked-arithmetic ceiling: `usize` is 64-bit on the target, so a product
    or sum reaching `2^64` is the overflow `checked_mul` / `checked_add` reject. -/
def usizeMax : Nat := 2 ^ 64

/-- The verdict, mirroring `Result<(usize, usize, usize), ElfError>`: the accepted
    triple, a wrong-entry-size error, or an out-of-bounds error (overflow or a
    table past the image end both land here, as in the Rust). -/
inductive Outcome where
  | ok (off size count : Nat)
  | errSize
  | errBounds
  deriving DecidableEq

/-- The bounds check, branch for branch. `dataLen` is the image length, `expected`
    is `size_of::<ProgramHeader>()`. -/
def check (dataLen phoff phsize phnum expected : Nat) : Outcome :=
  if phnum = 0 then .ok phoff phsize phnum
  else if phsize ≠ expected then .errSize
  else if phsize * phnum ≥ usizeMax then .errBounds
  else if phoff + phsize * phnum ≥ usizeMax then .errBounds
  else if phoff + phsize * phnum > dataLen then .errBounds
  else .ok phoff phsize phnum

/-- An empty program-header table is accepted with no bounds obligation: the
    loader reads nothing. -/
theorem empty_accepted (dataLen phoff phsize expected : Nat) :
    check dataLen phoff phsize 0 expected = .ok phoff phsize 0 := by
  simp [check]

/-- Characterization of an accepted non-empty table: the entry size is exactly
    the expected one, the byte span and its end both fit the address type, and
    the table ends at or before the image end. Every accepted-table property is
    read off this lemma. -/
theorem accepted_nonempty (dataLen phoff phsize phnum expected : Nat)
    (hn : phnum ≠ 0) (h : check dataLen phoff phsize phnum expected = .ok phoff phsize phnum) :
    phsize = expected ∧ phsize * phnum < usizeMax ∧
      phoff + phsize * phnum < usizeMax ∧ phoff + phsize * phnum ≤ dataLen := by
  unfold check at h
  rw [if_neg hn] at h
  by_cases hs : phsize ≠ expected
  · rw [if_pos hs] at h; exact absurd h (by simp)
  · rw [if_neg hs] at h
    by_cases hm : phsize * phnum ≥ usizeMax
    · rw [if_pos hm] at h; exact absurd h (by simp)
    · rw [if_neg hm] at h
      by_cases ha : phoff + phsize * phnum ≥ usizeMax
      · rw [if_pos ha] at h; exact absurd h (by simp)
      · rw [if_neg ha] at h
        by_cases he : phoff + phsize * phnum > dataLen
        · rw [if_pos he] at h; exact absurd h (by simp)
        · exact ⟨Decidable.of_not_not hs, by omega, by omega, by omega⟩

/-- The load-bearing safety fact: an accepted non-empty table lies wholly inside
    the image, so every one of its `phnum` entries is an in-bounds read. -/
theorem accepted_table_in_bounds (dataLen phoff phsize phnum expected : Nat)
    (hn : phnum ≠ 0) (h : check dataLen phoff phsize phnum expected = .ok phoff phsize phnum) :
    phoff + phsize * phnum ≤ dataLen :=
  (accepted_nonempty dataLen phoff phsize phnum expected hn h).2.2.2

/-- An accepted non-empty table has the exact entry size the loader expects, so
    the stride it reads with matches the format. -/
theorem accepted_exact_size (dataLen phoff phsize phnum expected : Nat)
    (hn : phnum ≠ 0) (h : check dataLen phoff phsize phnum expected = .ok phoff phsize phnum) :
    phsize = expected :=
  (accepted_nonempty dataLen phoff phsize phnum expected hn h).1

/-- An accepted non-empty table's span never wrapped the address type, so the
    end offset the loader computed is honest. -/
theorem accepted_no_overflow (dataLen phoff phsize phnum expected : Nat)
    (hn : phnum ≠ 0) (h : check dataLen phoff phsize phnum expected = .ok phoff phsize phnum) :
    phoff + phsize * phnum < usizeMax :=
  (accepted_nonempty dataLen phoff phsize phnum expected hn h).2.2.1

/-- A wrong entry size on a non-empty table is refused. -/
theorem wrong_size_rejected (dataLen phoff phsize phnum expected : Nat)
    (hn : phnum ≠ 0) (hs : phsize ≠ expected) :
    check dataLen phoff phsize phnum expected = .errSize := by
  unfold check; rw [if_neg hn, if_pos hs]

/-- A table that runs past the image end is refused (given a valid size and no
    overflow), so no read is ever admitted beyond the buffer. -/
theorem overrun_rejected (dataLen phoff phsize phnum expected : Nat)
    (hn : phnum ≠ 0) (hs : phsize = expected)
    (hm : ¬ phsize * phnum ≥ usizeMax) (ha : ¬ phoff + phsize * phnum ≥ usizeMax)
    (he : phoff + phsize * phnum > dataLen) :
    check dataLen phoff phsize phnum expected = .errBounds := by
  unfold check
  rw [if_neg hn, if_neg (not_not_intro hs), if_neg hm, if_neg ha, if_pos he]

end Nonos.ElfPhdr
