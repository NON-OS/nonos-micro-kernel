/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Nonce freshness. A nonce generator hands out a strictly increasing sequence.
The theorems below show each issue advances the counter, an issued nonce is
strictly below the next one, and two successive issues are distinct, so a
nonce is never reused, the freshness the attestation and replay defenses need.
-/

namespace Nonos.Nonce

/-- A nonce generator: the next value to hand out. -/
structure Gen where
  next : Nat

/-- Issue a nonce and advance the generator. -/
def issue (g : Gen) : Nat × Gen := (g.next, ⟨g.next + 1⟩)

/-- Issuing advances the counter by one. -/
theorem issue_advances (g : Gen) : (issue g).2.next = g.next + 1 := by simp only [issue]

/-- The issued value is the old counter. -/
theorem issue_value (g : Gen) : (issue g).1 = g.next := by simp only [issue]

/-- An issued nonce is strictly below the advanced counter, so no later issue
    can repeat it. -/
theorem issued_lt_next (g : Gen) : (issue g).1 < (issue g).2.next := by
  simp only [issue]; omega

/-- Issue two nonces in succession. -/
def issue2 (g : Gen) : Nat × Nat × Gen :=
  ((issue g).1, (issue (issue g).2).1, (issue (issue g).2).2)

/-- Two successive issues are distinct: no nonce is ever reused. -/
theorem issue2_distinct (g : Gen) : (issue2 g).1 ≠ (issue2 g).2.1 := by
  simp only [issue2, issue]; omega

/-- The second issue is strictly greater than the first: the sequence is
    strictly increasing. -/
theorem issue2_increasing (g : Gen) : (issue2 g).1 < (issue2 g).2.1 := by
  simp only [issue2, issue]; omega

/-- Issue `n` nonces, returning the final generator. -/
def issueN (g : Gen) : Nat → Gen
  | 0 => g
  | n + 1 => issueN (issue g).2 n

/-- After issuing `n` nonces the counter has advanced by exactly `n`: the
    generator never stalls or repeats a value. -/
theorem issueN_advances (g : Gen) (n : Nat) : (issueN g n).next = g.next + n := by
  induction n generalizing g with
  | zero => simp [issueN]
  | succ k ih => simp only [issueN]; rw [ih (issue g).2]; simp only [issue]; omega

end Nonos.Nonce
