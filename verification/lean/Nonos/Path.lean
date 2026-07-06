/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification-level proof of filesystem path safety: a path can never escape
above its root through `..`. The real VFS path canonicalization is proven to
match by the Kani + runnable harnesses in `userland/fs_proofs` (no `..`
component survives, adjacent slashes collapse, the `/capsules` root is
enforced). Lean proves the property on the resolution model; the code proofs
show the canonicalizer implements it.
-/

namespace Nonos.Path

/-- A path component after splitting on '/'. -/
inductive Component where
  | up    -- ".."
  | cur   -- "."
  | name  -- an ordinary name

/-- Resolving a component against the current depth below the root. `..` uses
    saturating subtraction, so at the root it is a no-op: a path cannot escape
    above where it started. -/
def depthStep (d : Nat) : Component → Nat
  | .up   => d - 1
  | .cur  => d
  | .name => d + 1

/-- The depth reached after resolving a whole path from its root. -/
def resolve (cs : List Component) : Nat := cs.foldl depthStep 0

/-- `..` only ever moves toward the root, never deeper. -/
theorem up_moves_toward_root (d : Nat) : depthStep d .up ≤ d :=
  Nat.sub_le d 1

/-- At the root, `..` stays at the root: no upward escape. -/
theorem dotdot_at_root_is_noop : depthStep 0 .up = 0 :=
  rfl

/-- A `..` at the very start of a path (i.e. at the root) is neutralized: it
    resolves the same as if it were not there. -/
theorem leading_dotdot_neutralized (cs : List Component) :
    resolve (.up :: cs) = resolve cs :=
  rfl

/-- An ordinary name descends exactly one level. -/
theorem name_descends_one (d : Nat) : depthStep d .name = d + 1 :=
  rfl

/-- No single step ever moves more than one level in either direction, so the
    resolved depth changes by at most one per component. -/
theorem step_moves_at_most_one (d : Nat) (c : Component) :
    depthStep d c ≤ d + 1 ∧ d ≤ depthStep d c + 1 := by
  cases c <;> simp [depthStep] <;> omega

/-- A path made only of `..` components resolves to the root, never below it: no
    amount of parent references can escape upward. -/
theorem all_up_stays_at_root : ∀ n : Nat, resolve (List.replicate n .up) = 0
  | 0 => rfl
  | n + 1 => by
    show resolve (.up :: List.replicate n .up) = 0
    rw [leading_dotdot_neutralized]
    exact all_up_stays_at_root n

end Nonos.Path
