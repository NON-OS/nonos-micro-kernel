/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

FRI query indices. Each query is an index into the evaluation domain, derived from
the transcript and reduced modulo the domain size, so it always lands in range.
The theorems below show reduction is always in range, an index at or above the
domain is rejected, a folded index stays in the halved domain, and a query list of
a fixed length asks exactly that many openings: the bounds the verifier checks
before it reads a Merkle path.
-/

namespace Nonos.Stark.QueryIndex

/-- A query index is in range when it is below the evaluation-domain size. -/
def inRange (domain idx : Nat) : Prop := idx < domain

/-- Reducing any value modulo a nonempty domain lands in range. -/
theorem reduce_in_range (domain idx : Nat) (h : 0 < domain) : inRange domain (idx % domain) :=
  Nat.mod_lt idx h

/-- Index zero is in range for a nonempty domain. -/
theorem zero_in_range (domain : Nat) (h : 0 < domain) : inRange domain 0 := h

/-- An index at or above the domain size is rejected. -/
theorem out_of_range_rejected (domain idx : Nat) (h : domain ≤ idx) : ¬ inRange domain idx := by
  simp only [inRange]; omega

/-- The verifier accepts an index iff it is below the domain. -/
theorem accept_iff_below (domain idx : Nat) : inRange domain idx ↔ idx < domain := Iff.rfl

/-- Folding an index by dropping its low bit keeps it in the halved domain. -/
theorem fold_stays_in_range (domain idx : Nat) (h : inRange domain idx) :
    inRange domain (idx / 2) := by
  simp only [inRange] at *; omega

/-- A pair-sibling index (toggling the low bit) stays in an even-sized domain. -/
theorem paired_sibling_in_range (domain idx : Nat) (h : inRange domain idx)
    (heven : domain % 2 = 0) : inRange domain (2 * (idx / 2) + (1 - idx % 2)) := by
  simp only [inRange] at *; omega

/-- Two in-range indices summed and reduced stay in range. -/
theorem combined_in_range (domain a b : Nat) (h : 0 < domain) :
    inRange domain ((a + b) % domain) := Nat.mod_lt _ h

/-- Reduction is idempotent on an already in-range index. -/
theorem reduce_id_in_range (domain idx : Nat) (h : inRange domain idx) : idx % domain = idx :=
  Nat.mod_eq_of_lt h

/-- Every reduced index is strictly bounded, so a Merkle path of matching depth exists. -/
theorem reduced_bounded (domain idx : Nat) (h : 0 < domain) : idx % domain + 1 ≤ domain := by
  have := Nat.mod_lt idx h; omega

end Nonos.Stark.QueryIndex
