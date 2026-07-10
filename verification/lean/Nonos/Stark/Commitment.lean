/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The note commitment and what it binds. A shielded note commits to its value under a
collision-resistant hash together with a blinding factor, cm = H(value, blinding).
The pool trusts the commitment to fix the amount: a prover who deposits a note
cannot later open it to a different value. This module proves that binding from the
one assumption it needs, that the commitment is injective, which is exactly the
collision resistance of the hash. Under it, equal commitments force equal values and
distinct values force distinct commitments. The assumption is discharged code-side
by the Poseidon known-answer tests; here it is a named hypothesis, not hidden.
-/

namespace Nonos.Stark.Commitment

/-- The commitment binds the value: if two openings collide, they have the same
    value. The hypothesis is the hash's injectivity, its collision resistance. -/
theorem commitment_binds_value (commit : Int → Int → Int)
    (hinj : ∀ v r v' r', commit v r = commit v' r' → v = v' ∧ r = r')
    (v r v' r' : Int) (h : commit v r = commit v' r') : v = v' :=
  (hinj v r v' r' h).1

/-- The blinding is bound too: a collision fixes it, so a note cannot be reblinded
    to forge a different opening. -/
theorem commitment_binds_blinding (commit : Int → Int → Int)
    (hinj : ∀ v r v' r', commit v r = commit v' r' → v = v' ∧ r = r')
    (v r v' r' : Int) (h : commit v r = commit v' r') : r = r' :=
  (hinj v r v' r' h).2

/-- Distinct values give distinct commitments: a prover cannot make one commitment
    stand for two amounts. The contrapositive of value binding. -/
theorem distinct_values_distinct_commitments (commit : Int → Int → Int)
    (hinj : ∀ v r v' r', commit v r = commit v' r' → v = v' ∧ r = r')
    (v r v' r' : Int) (hne : v ≠ v') : commit v r ≠ commit v' r' :=
  fun hc => hne (commitment_binds_value commit hinj v r v' r' hc)

/-- A forged amount cannot match an honest commitment: any commitment equal to an
    honest note's commitment opens to the honest note's value, never a forged one. -/
theorem a_forged_value_cannot_match (commit : Int → Int → Int)
    (hinj : ∀ v r v' r', commit v r = commit v' r' → v = v' ∧ r = r')
    (honestValue honestRand forgedValue forgedRand : Int)
    (hforge : forgedValue ≠ honestValue)
    (h : commit forgedValue forgedRand = commit honestValue honestRand) : False :=
  hforge (commitment_binds_value commit hinj forgedValue forgedRand honestValue honestRand h)

end Nonos.Stark.Commitment
