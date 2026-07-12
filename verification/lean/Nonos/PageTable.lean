/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Page-table W⊕X preservation. A permission is safe when it is not both writable
and executable. The theorems below show that installing a safe permission keeps
the table safe, that a checked map refuses a W+X permission outright, and that
any sequence of checked maps preserves the no-W-X invariant across the whole
table, so no map/unmap trace can ever open a writable-executable page.
-/

namespace Nonos.PageTable

/-- A page permission: the writable and executable bits of a PTE. -/
structure Perm where
  w : Bool
  x : Bool

/-- A permission is safe when it is not both writable and executable. -/
def Perm.safe (p : Perm) : Prop := ¬ (p.w = true ∧ p.x = true)

/-- A page table: a permission per page index. -/
def Table := Nat → Perm

/-- The whole table is W⊕X: every page is safe. -/
def Safe (t : Table) : Prop := ∀ i, (t i).safe

/-- Install a permission for one page (the kernel writes a PTE). -/
def map (t : Table) (i : Nat) (p : Perm) : Table :=
  fun j => if j = i then p else t j

/-- Installing a safe permission into a safe table keeps it safe. -/
theorem map_safe (t : Table) (i : Nat) (p : Perm)
    (ht : Safe t) (hp : p.safe) : Safe (map t i p) := by
  intro j
  unfold map
  by_cases h : j = i
  · rw [if_pos h]; exact hp
  · rw [if_neg h]; exact ht j

/-- A checked map: the loader refuses a permission that is both W and X,
    leaving the table unchanged rather than opening a W+X page. -/
def mapChecked (t : Table) (i : Nat) (p : Perm) : Table :=
  if p.w && p.x then t else map t i p

/-- A checked map always preserves W⊕X, whatever permission is requested: an
    unsafe request is refused, a safe one is installed. -/
theorem mapChecked_safe (t : Table) (i : Nat) (p : Perm) (ht : Safe t) :
    Safe (mapChecked t i p) := by
  unfold mapChecked
  by_cases h : (p.w && p.x) = true
  · rw [if_pos h]; exact ht
  · rw [if_neg h]
    refine map_safe t i p ht ?_
    unfold Perm.safe
    intro hcon
    apply h
    simp [hcon.1, hcon.2]

/-- A whole sequence of page installs. -/
def mapAll (t : Table) : List (Nat × Perm) → Table
  | [] => t
  | (i, p) :: rest => mapAll (map t i p) rest

/-- Any sequence of installs, each with a safe permission, preserves W⊕X. -/
theorem mapAll_safe (t : Table) (ops : List (Nat × Perm))
    (ht : Safe t) (hops : ∀ op ∈ ops, op.2.safe) : Safe (mapAll t ops) := by
  induction ops generalizing t with
  | nil => exact ht
  | cons op rest ih =>
    apply ih
    · exact map_safe t op.1 op.2 ht (hops op (List.mem_cons_self op rest))
    · intro x hx; exact hops x (List.mem_cons_of_mem op hx)

/-- A whole sequence of CHECKED installs preserves W⊕X unconditionally: the
    loader may be handed any permissions at all and no page ends up W+X. -/
def mapAllChecked (t : Table) : List (Nat × Perm) → Table
  | [] => t
  | (i, p) :: rest => mapAllChecked (mapChecked t i p) rest

theorem mapAllChecked_safe (t : Table) (ops : List (Nat × Perm)) (ht : Safe t) :
    Safe (mapAllChecked t ops) := by
  induction ops generalizing t with
  | nil => exact ht
  | cons op rest ih => exact ih (mapChecked t op.1 op.2) (mapChecked_safe t op.1 op.2 ht)

end Nonos.PageTable
