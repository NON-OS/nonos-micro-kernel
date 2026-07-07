/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification of the capsule ELF loader bounds. The loader reads an ELF header
and program-header table from an attacker-supplied image; the security property
is that an accepted table lies entirely inside the file, so no program-header
read can escape the mapped image. `userland/kernel_proofs` discharges this on
the real header parsers: `short_headers_are_rejected_not_panicked` and
`program_header_table_never_overflows_or_escapes_the_file` in `elf_tests.rs`,
over adversarial headers with large offsets and counts. The code proofs also
show the `u64` arithmetic `phoff + phnum * phentsize` never wraps, which is
what entitles this model to plain natural-number arithmetic.
-/

namespace Nonos.Loader

/-- The header fields the bounds check reads: the header's own size, the
    program-header table offset, entry count, entry size, and the file length. -/
structure Header where
  ehsize : Nat
  phoff : Nat
  phnum : Nat
  phentsize : Nat

/-- The loader's acceptance gate: the header itself fits, and the whole
    program-header table ends at or before the end of the file. -/
def accepts (h : Header) (filelen : Nat) : Bool :=
  h.ehsize ≤ filelen && h.phoff + h.phnum * h.phentsize ≤ filelen

/-- A file shorter than the header is rejected, never sliced. -/
theorem truncated_header_rejected (h : Header) (filelen : Nat)
    (hshort : filelen < h.ehsize) : accepts h filelen = false := by
  unfold accepts
  have : ¬ h.ehsize ≤ filelen := by omega
  simp [this]

/-- An accepted program-header table lies entirely inside the file. -/
theorem accepted_table_inside_file (h : Header) (filelen : Nat)
    (hacc : accepts h filelen = true) :
    h.phoff + h.phnum * h.phentsize ≤ filelen := by
  unfold accepts at hacc
  simp [Bool.and_eq_true] at hacc
  exact hacc.2

/-- Every individual program-header entry of an accepted table lies inside the
    file: entry `i` ends at `phoff + (i + 1) * phentsize`, which the table
    bound dominates. A loader that reads any entry of an accepted table cannot
    read past the file. -/
theorem accepted_entry_inside_file (h : Header) (filelen i : Nat)
    (hacc : accepts h filelen = true) (hi : i < h.phnum) :
    h.phoff + (i + 1) * h.phentsize ≤ filelen := by
  have htab := accepted_table_inside_file h filelen hacc
  have : (i + 1) * h.phentsize ≤ h.phnum * h.phentsize :=
    Nat.mul_le_mul_right _ (by omega)
  omega

end Nonos.Loader
