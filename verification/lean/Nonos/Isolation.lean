/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification-level proofs of the memory-isolation properties. The real kernel
code that implements them is proven to match by the runnable + Kani harnesses in
`userland/kernel_proofs` (the `to_pte_flags` encoding and the `check_range`
bounds). Lean proves the property; the code proofs discharge it on the
implementation.
-/

namespace Nonos.Isolation

/-- The page-permission bits that decide W^X. -/
structure Perm where
  write : Bool
  execute : Bool

/-- A W^X violation: writable and executable at once. -/
def IsWxViolation (p : Perm) : Prop := p.write = true ∧ p.execute = true

/-- The page-table entry is writable iff WRITE is set and executable iff EXECUTE
    is set (NX is applied otherwise). -/
def pteWritable (p : Perm) : Bool := p.write
def pteExecutable (p : Perm) : Bool := p.execute

/-- A permission set that is not a W^X violation never encodes a page that is
    both writable and executable, so a mapper that rejects `IsWxViolation`
    cannot install a W+X page. -/
theorem no_wx_page (p : Perm) (h : ¬ IsWxViolation p) :
    ¬ (pteWritable p = true ∧ pteExecutable p = true) := by
  unfold IsWxViolation pteWritable pteExecutable at *
  exact h

/-- The encoding is exact: a page is writable and executable at once if and only
    if its permission is a W^X violation. So rejecting `IsWxViolation` is both
    necessary and sufficient to keep W+X pages off the tables. -/
theorem wx_page_iff_violation (p : Perm) :
    (pteWritable p = true ∧ pteExecutable p = true) ↔ IsWxViolation p := by
  unfold IsWxViolation pteWritable pteExecutable
  tauto

/-- The last user address and the maximum single-copy length. -/
def userEnd : Nat := 0x7FFFFFFFFFFF
def maxCopy : Nat := 64 * 1024 * 1024

/-- A user copy is accepted only if it is non-null, within the size limit, and
    the whole range fits inside user space without wrapping. -/
def Accepts (addr len : Nat) : Prop :=
  addr ≠ 0 ∧ len ≤ maxCopy ∧ addr + len ≤ userEnd + 1

/-- Every byte of an accepted user copy lands inside user space. -/
theorem accepted_stays_in_user_space (addr len i : Nat)
    (h : Accepts addr len) (hi : i < len) : addr + i ≤ userEnd := by
  obtain ⟨_, _, hbound⟩ := h
  omega

end Nonos.Isolation
