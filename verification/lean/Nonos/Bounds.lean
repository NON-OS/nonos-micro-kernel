/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Buffer-access bounds checking. An access of `len` bytes at offset `off` into a
buffer of `size` bytes is safe when it stays inside the buffer. The theorems
below show a safe access's first and last bytes are in range, every index of a
safe access lands inside the buffer, shrinking a safe access stays safe, and an
oversized access is refused, the spatial-safety facts the copy paths rely on.

The kernel's ELF relocator gates each write with the range test in
`src/elf/reloc/apply/range.rs`, which `in_segment` delegates to. The
`mechanism_proofs` crate includes that file and proves with Kani that an
in-range access sits wholly inside the segment with neither end overflowing, so
this spatial check is proven of the code the loader runs.
-/

namespace Nonos.Bounds

/-- An access of `len` bytes at `off` into a buffer of `size` bytes. -/
structure Access where
  off : Nat
  len : Nat
  size : Nat

/-- The access stays inside the buffer. -/
def safe (a : Access) : Prop := a.off + a.len ≤ a.size

/-- The first byte of a nonempty safe access is in range. -/
theorem safe_start_in_bounds (a : Access) (h : safe a) (hl : 0 < a.len) :
    a.off < a.size := by simp only [safe] at h; omega

/-- The last byte of a nonempty safe access is in range. -/
theorem safe_last_in_bounds (a : Access) (h : safe a) (hl : 0 < a.len) :
    a.off + a.len - 1 < a.size := by simp only [safe] at h; omega

/-- An access whose end passes the buffer is not safe. -/
theorem oversized_unsafe (a : Access) (h : a.size < a.off + a.len) : ¬ safe a := by
  simp only [safe]; omega

/-- A zero-length access anywhere within the buffer is safe. -/
theorem zero_len_safe (off size : Nat) (h : off ≤ size) : safe ⟨off, 0, size⟩ := by
  simp only [safe]; omega

/-- Shrinking a safe access keeps it safe. -/
theorem shrink_safe (a : Access) (l : Nat) (h : safe a) (hl : l ≤ a.len) :
    safe ⟨a.off, l, a.size⟩ := by simp only [safe] at *; omega

/-- Byte `i` of the access. -/
def indexes (a : Access) (i : Nat) : Prop := i < a.len

/-- Every byte of a safe access lands inside the buffer: no out-of-bounds. -/
theorem index_in_buffer (a : Access) (i : Nat) (hs : safe a) (hi : indexes a i) :
    a.off + i < a.size := by
  simp only [safe] at hs; simp only [indexes] at hi; omega

/-- Safety is monotone in the buffer size: a larger buffer only ever keeps a
    safe access safe. -/
theorem grow_buffer_safe (a : Access) (n : Nat) (h : safe a) :
    safe ⟨a.off, a.len, a.size + n⟩ := by simp only [safe] at *; omega

end Nonos.Bounds
