/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Uniqueness of the low-degree extension, the Reed-Solomon fact the whole argument
system rests on. Two polynomials of length at most n that agree on n distinct
points are equal as functions: their difference has length at most n and vanishes
on n distinct points, so the numeric root bound forces it to zero. This is why a
codeword is pinned to a single low-degree polynomial once it agrees at more
positions than the degree, which is what FRI proximity and the trace consistency
checks turn disagreement into a contradiction against. It is the deterministic
core under the Schwartz-Zippel bound, no probability, no assumption.
-/

import Nonos.Stark.Polynomial
import Nonos.Stark.Fold

namespace Nonos.Stark.Extension

open Nonos.Stark.Polynomial Nonos.Stark.Fold

/-- Subtraction cannot lengthen: the difference is as long as the longer input. -/
theorem sub_length (p q : Poly) : (sub p q).length = max p.length q.length := by
  rw [sub, add_length, scale_length]

/-- Uniqueness of the low-degree extension: two polynomials of length at most the
    number of agreement points, agreeing on those distinct points, are equal
    everywhere. The difference vanishes on as many distinct points as its length,
    so the numeric root bound collapses it to zero. -/
theorem low_degree_extension_is_unique (p q : Poly) (rs : List Int)
    (hnd : rs.Nodup) (hp : p.length ≤ rs.length) (hq : q.length ≤ rs.length)
    (hagree : ∀ r ∈ rs, eval p r = eval q r) :
    ∀ x, eval p x = eval q x := by
  have hz : ∀ x, eval (sub p q) x = 0 := by
    apply root_bound (sub p q) rs hnd
    · intro r hr; rw [eval_sub]; have := hagree r hr; omega
    · rw [sub_length]; omega
  intro x
  have hx := hz x
  rw [eval_sub] at hx
  omega

/-- The contrapositive the proximity test uses: two low-degree polynomials that
    differ anywhere disagree at more points than their degree, so they cannot
    agree on a full set of that many distinct evaluation points. -/
theorem distinct_low_degree_polys_disagree (p q : Poly) (rs : List Int)
    (hnd : rs.Nodup) (hp : p.length ≤ rs.length) (hq : q.length ≤ rs.length)
    (x : Int) (hne : eval p x ≠ eval q x) :
    ∃ r ∈ rs, eval p r ≠ eval q r := by
  apply Classical.byContradiction
  intro h
  apply hne
  apply low_degree_extension_is_unique p q rs hnd hp hq _ x
  intro r hr
  apply Classical.byContradiction
  intro hrne
  exact h ⟨r, hr, hrne⟩

end Nonos.Stark.Extension
