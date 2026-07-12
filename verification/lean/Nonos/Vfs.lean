/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

VFS path confinement. A path resolves as a depth below a root: a name descends,
`..` ascends but is clamped at the root. The theorems below show `..` never
takes you below the root, a path of only `..` from the root stays at the root,
and resolution never conjures more depth than the names it descended, so a
path can never escape its root.
-/

namespace Nonos.Vfs

/-- A path component: a name descends one level, `dotdot` ascends one. -/
inductive Comp
  | name
  | dotdot

/-- The depth after applying one component, clamped at the root (Nat floor). -/
def apply (d : Nat) : Comp → Nat
  | Comp.name => d + 1
  | Comp.dotdot => d - 1

/-- Resolve a path from a starting depth. -/
def resolve (d : Nat) : List Comp → Nat
  | [] => d
  | c :: rest => resolve (apply d c) rest

/-- `..` never increases depth. -/
theorem apply_dotdot_le (d : Nat) : apply d Comp.dotdot ≤ d := by simp only [apply]; omega

/-- A name descends exactly one level. -/
theorem apply_name (d : Nat) : apply d Comp.name = d + 1 := by simp only [apply]

/-- `..` at the root stays at the root: you cannot escape below `/`. -/
theorem dotdot_at_root : apply 0 Comp.dotdot = 0 := by simp only [apply]

/-- Resolving only `..` components never rises above the starting depth. -/
theorem resolve_dotdots_le (d n : Nat) :
    resolve d (List.replicate n Comp.dotdot) ≤ d := by
  induction n generalizing d with
  | zero => simp [resolve, List.replicate]
  | succ k ih =>
    simp only [List.replicate, resolve]
    have h1 := ih (apply d Comp.dotdot)
    have h2 := apply_dotdot_le d
    omega

/-- A path of only `..` from the root stays at the root: no escape. -/
theorem resolve_dotdots_root (n : Nat) :
    resolve 0 (List.replicate n Comp.dotdot) = 0 := by
  have := resolve_dotdots_le 0 n
  omega

/-- Resolution never exceeds the starting depth plus the number of components:
    a path cannot conjure depth it did not descend. -/
theorem resolve_le (d : Nat) (cs : List Comp) :
    resolve d cs ≤ d + cs.length := by
  induction cs generalizing d with
  | nil => simp [resolve]
  | cons c rest ih =>
    simp only [resolve, List.length]
    cases c with
    | name =>
      have hih := ih (apply d Comp.name)
      simp only [apply] at hih ⊢
      omega
    | dotdot =>
      have hih := ih (apply d Comp.dotdot)
      simp only [apply] at hih ⊢
      omega

end Nonos.Vfs
