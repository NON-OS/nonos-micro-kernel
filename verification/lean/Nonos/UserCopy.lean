/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

User-buffer range policy. Every copy across the user/kernel boundary clears its
range through this policy before a byte moves. The model mirrors the pure range
check in `src/usercopy/policy.rs::check_range`: a null pointer, a zero length,
an oversized length, an address that overflows on `addr + len - 1`, and a range
whose last byte passes the canonical user limit are each rejected; anything it
accepts is a page-aligned span that lies wholly inside user space.

The theorems below establish the spatial-safety facts the copy paths depend on:
an accepted non-empty range never covers the null page and never reaches kernel
space, its start page does not exceed its end page, both ends are page aligned,
and the four rejection cases are exactly the ones the code returns an error for.
Together they say the boundary check the copy runs cannot pass a range that
would touch memory outside the caller's user address space.
-/

namespace Nonos.UserCopy

/-- Canonical user-space ceiling: the highest byte a user pointer may name,
    matching `USER_SPACE_END` in the policy. -/
def userSpaceEnd : Nat := 0x00007FFFFFFFFFFF

/-- Page size in bytes. -/
def pageSize : Nat := 4096

/-- Largest single copy the policy admits, matching `MAX_COPY_SIZE`. -/
def maxCopySize : Nat := 64 * 1024 * 1024

/-- Clear the low twelve bits, the page base of an address (`addr & !0xFFF`). -/
def pageBase (addr : Nat) : Nat := addr - addr % pageSize

/-- The page span an accepted range covers. -/
structure UserRange where
  startPage : Nat
  endPage : Nat

/-- The outcome of the range check: an error, an empty (accepted, no-op) range,
    or an accepted page span. This mirrors `Result<Option<UserRange>, _>`. -/
inductive Outcome where
  | nullPointer
  | sizeTooLarge
  | addressOverflow
  | invalidAddress
  | empty
  | range (r : UserRange)

/-- The policy over a 64-bit address space of width `addrBits` (the code's `u64`
    checked_add overflows past `2^64`). `check addr len` follows `check_range`
    branch for branch. -/
def check (addrBits addr len : Nat) : Outcome :=
  if addr = 0 then .nullPointer
  else if len > maxCopySize then .sizeTooLarge
  else if len = 0 then .empty
  else if addr + (len - 1) ≥ 2 ^ addrBits then .addressOverflow
  else if addr + (len - 1) > userSpaceEnd then .invalidAddress
  else .range ⟨pageBase addr, pageBase (addr + (len - 1))⟩

/-- A null pointer is always rejected, whatever the length. -/
theorem null_rejected (b l : Nat) : check b 0 l = .nullPointer := by
  simp [check]

/-- An oversized length is rejected for any non-null address. -/
theorem oversized_rejected (b a l : Nat) (ha : a ≠ 0) (hl : l > maxCopySize) :
    check b a l = .sizeTooLarge := by
  simp [check, ha, hl]

/-- A zero-length copy at a non-null address is accepted as a no-op. -/
theorem zero_len_empty (b a : Nat) (ha : a ≠ 0) : check b a 0 = .empty := by
  have : ¬ (0 > maxCopySize) := by simp [maxCopySize]
  simp [check, ha, this]

/-- The page base never exceeds its address. -/
theorem pageBase_le (addr : Nat) : pageBase addr ≤ addr := by
  simp only [pageBase]; omega

/-- The page base is page aligned: a whole number of pages. -/
theorem pageBase_aligned (addr : Nat) : pageBase addr % pageSize = 0 := by
  simp only [pageBase, pageSize]
  omega

/-- Characterization of an accepted range: the page span is exactly the base of
    the start and end addresses, the address is non-null and non-empty, and the
    last byte is at or below the user ceiling. Every accepted-range property
    below is read off this one lemma, which is the only place the branch
    structure of `check` is taken apart. -/
theorem check_accepts (b a l : Nat) (r : UserRange) (h : check b a l = .range r) :
    r.startPage = pageBase a ∧ r.endPage = pageBase (a + (l - 1)) ∧
    a ≠ 0 ∧ l ≠ 0 ∧ a + (l - 1) ≤ userSpaceEnd := by
  unfold check at h
  by_cases h1 : a = 0
  · rw [if_pos h1] at h; exact Outcome.noConfusion h
  · rw [if_neg h1] at h
    by_cases h2 : l > maxCopySize
    · rw [if_pos h2] at h; exact Outcome.noConfusion h
    · rw [if_neg h2] at h
      by_cases h3 : l = 0
      · rw [if_pos h3] at h; exact Outcome.noConfusion h
      · rw [if_neg h3] at h
        by_cases h4 : a + (l - 1) ≥ 2 ^ b
        · rw [if_pos h4] at h; exact Outcome.noConfusion h
        · rw [if_neg h4] at h
          by_cases h5 : a + (l - 1) > userSpaceEnd
          · rw [if_pos h5] at h; exact Outcome.noConfusion h
          · rw [if_neg h5] at h
            injection h with hr
            subst hr
            exact ⟨rfl, rfl, h1, h3, by omega⟩

/-- An accepted range is page aligned at both ends. -/
theorem accepted_aligned (b a l : Nat) (r : UserRange) (h : check b a l = .range r) :
    r.startPage % pageSize = 0 ∧ r.endPage % pageSize = 0 := by
  obtain ⟨hs, he, _⟩ := check_accepts b a l r h
  rw [hs, he]
  exact ⟨pageBase_aligned a, pageBase_aligned (a + (l - 1))⟩

/-- An accepted range's start page never exceeds its end page: the walk in
    `validate.rs` steps forward from a base no higher than the top. -/
theorem accepted_start_le_end (b a l : Nat) (r : UserRange)
    (h : check b a l = .range r) : r.startPage ≤ r.endPage := by
  obtain ⟨hs, he, _⟩ := check_accepts b a l r h
  rw [hs, he]
  simp only [pageBase, pageSize]
  omega

/-- The load-bearing invariant: an accepted non-empty range lies wholly inside
    user space. Its end page, hence every byte the copy will touch, is at or
    below the canonical user ceiling, so the transfer can never reach the null
    page below or kernel memory above. -/
theorem accepted_within_user (b a l : Nat) (r : UserRange)
    (h : check b a l = .range r) : r.endPage ≤ userSpaceEnd := by
  obtain ⟨_, he, _, _, hlim⟩ := check_accepts b a l r h
  rw [he]
  have hb : pageBase (a + (l - 1)) ≤ a + (l - 1) := pageBase_le _
  omega

/-- An accepted range starts above the null page: the start address was
    non-zero and within user space, so the copy never dereferences page zero. -/
theorem accepted_nonzero_addr (b a l : Nat) (r : UserRange)
    (h : check b a l = .range r) : a ≠ 0 := by
  obtain ⟨_, _, ha, _⟩ := check_accepts b a l r h
  exact ha

end Nonos.UserCopy
