/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

How long a delegated capability lasts.

`create_delegation` signs the result with the kernel key, so a delegation that
outlives its parent verifies everywhere for as long as it lasts. There is no
revocation sweep behind it to catch the mistake. The meet in
`src/capabilities/delegation/lifetime.rs` is the whole mechanism, and it is
three lines with two edge cases that are easy to get backwards.

`None` means no expiry. The two cases worth naming: a parent with no expiry
imposes no bound, so the request stands unchanged; a parent with one always
bounds the result, including when the caller asked for nothing, which is the
case a `map` rather than a `map_or` would silently drop.

`mechanism_proofs` checks the real Rust against this shape under Kani for
every pair of inputs.
-/

namespace Nonos.Delegation

/-- An expiry in milliseconds, or none. -/
abbrev Expiry := Option Nat

/-- `delegation_expiry`: the earlier of what was asked for and what the parent
    has left. -/
def expiry (requested parent : Expiry) : Expiry :=
  match parent with
  | some p =>
    some (match requested with
          | some r => if r < p then r else p
          | none => p)
  | none => requested

/-- A delegation is live at `t` when it has no expiry or has not reached it. -/
def liveAt (e : Expiry) (t : Nat) : Prop :=
  match e with
  | some x => t < x
  | none => True

/-! ### The child never outlasts the parent -/

/-- **The delegation lifetime property.** A parent with an expiry always bounds
    the child, whatever was requested and even when nothing was.

    False if the meet takes the maximum, or if the `none` request case returns
    `none` instead of the parent's bound. -/
theorem never_outlives_parent (requested : Expiry) (p : Nat) :
    ∃ e, expiry requested (some p) = some e ∧ e ≤ p := by
  cases requested with
  | none => exact ⟨p, rfl, Nat.le_refl p⟩
  | some r =>
    by_cases h : r < p
    · exact ⟨r, by simp [expiry, h], Nat.le_of_lt h⟩
    · exact ⟨p, by simp [expiry, h], Nat.le_refl p⟩

/-- The child never outlasts what was asked for either, so a delegation cannot
    be quietly extended past the caller's own intent. -/
theorem never_outlasts_request (r : Nat) (parent : Expiry) :
    ∃ e, expiry (some r) parent = some e ∧ e ≤ r := by
  cases parent with
  | none => exact ⟨r, rfl, Nat.le_refl r⟩
  | some p =>
    by_cases h : r < p
    · exact ⟨r, by simp [expiry, h], Nat.le_refl r⟩
    · exact ⟨p, by simp [expiry, h], Nat.le_of_not_lt h⟩

/-- Anything live under the child is live under the parent: the child's window
    is contained in the parent's, which is the property the signature makes
    load bearing. -/
theorem live_child_implies_live_parent (requested : Expiry) (p t : Nat)
    (h : liveAt (expiry requested (some p)) t) : liveAt (some p) t := by
  obtain ⟨e, he, hle⟩ := never_outlives_parent requested p
  rw [he] at h
  exact Nat.lt_of_lt_of_le h hle

/-- A parent without an expiry imposes no bound: the request stands. -/
theorem unbounded_parent_keeps_request (requested : Expiry) :
    expiry requested none = requested := rfl

/-- A parent with an expiry never yields an unbounded child, which is the
    failure a plain `map` over the request would introduce. -/
theorem bounded_parent_never_yields_none (requested : Expiry) (p : Nat) :
    expiry requested (some p) ≠ none := by
  obtain ⟨e, he, _⟩ := never_outlives_parent requested p
  rw [he]
  exact Option.noConfusion

end Nonos.Delegation
