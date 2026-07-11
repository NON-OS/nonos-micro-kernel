/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

File-descriptor table safety. Each descriptor maps to a resource, or is closed.
The theorems below show open makes a descriptor point at exactly the resource
given and disturbs no other, close makes precisely that descriptor unusable and
no other, and a closed descriptor resolves to nothing, so a descriptor never
leaks the wrong resource and close is exact.
-/

namespace Nonos.Fd

/-- The descriptor table: each fd maps to a resource id, or `none` if closed. -/
def Table := Nat → Option Nat

/-- A descriptor is open when it resolves to a resource. -/
def isOpen (t : Table) (fd : Nat) : Prop := (t fd).isSome = true

/-- Open a descriptor onto a resource. -/
def open_ (t : Table) (fd res : Nat) : Table := fun x => if x = fd then some res else t x

/-- Close a descriptor. -/
def close (t : Table) (fd : Nat) : Table := fun x => if x = fd then none else t x

/-- The resource a descriptor resolves to. -/
def resolve (t : Table) (fd : Nat) : Option Nat := t fd

/-- Opening resolves the descriptor to exactly the resource given. -/
theorem open_resolves (t : Table) (fd res : Nat) : resolve (open_ t fd res) fd = some res := by
  simp only [resolve, open_]; simp

/-- An opened descriptor is open. -/
theorem open_isOpen (t : Table) (fd res : Nat) : isOpen (open_ t fd res) fd := by
  simp only [isOpen, open_]; simp

/-- Opening one descriptor disturbs no other. -/
theorem open_preserves (t : Table) (fd gd res : Nat) (hne : gd ≠ fd) :
    resolve (open_ t fd res) gd = resolve t gd := by
  simp only [resolve, open_]; simp only [if_neg hne]

/-- A closed descriptor resolves to nothing. -/
theorem close_resolves_none (t : Table) (fd : Nat) : resolve (close t fd) fd = none := by
  simp only [resolve, close]; simp

/-- A closed descriptor is not open. -/
theorem close_not_open (t : Table) (fd : Nat) : ¬ isOpen (close t fd) fd := by
  simp only [isOpen, close]; simp

/-- Closing one descriptor disturbs no other. -/
theorem close_preserves (t : Table) (fd gd : Nat) (hne : gd ≠ fd) :
    resolve (close t fd) gd = resolve t gd := by
  simp only [resolve, close]; simp only [if_neg hne]

/-- Reopening a closed descriptor makes it open again. -/
theorem close_open_id (t : Table) (fd res : Nat) :
    resolve (open_ (close t fd) fd res) fd = some res := by
  simp only [resolve, open_]; simp

end Nonos.Fd
