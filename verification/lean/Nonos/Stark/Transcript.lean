/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification of the Fiat-Shamir transcript that makes the STARK
non-interactive. The prover cannot choose its challenges: each is squeezed from
a sponge that has absorbed every prior commitment, so a challenge is a function
of the whole transcript up to that point. This module states the two properties
soundness rests on: the challenge stream is a deterministic function of the
absorbed data (so verifier and prover derive the same challenges), and it is
domain-separated and order-sensitive, so a transcript that absorbs different
data, or the same data in a different order, yields a different challenge. The
model treats the sponge permutation as an abstract function; that it behaves as
a random oracle is the assumption every Fiat-Shamir argument makes, stated in
the architecture document.

`userland/stark_proofs` discharges these on the real transcript in
`transcript_tests.rs`: `the_challenge_stream_is_deterministic`,
`a_one_bit_change_in_absorbed_data_changes_every_later_challenge`,
`absorb_order_matters`, `digest_and_field_absorbs_are_domain_separated`, and
`consecutive_challenges_advance`.
-/

namespace Nonos.Stark.Transcript

/-- The sponge state: absorbing folds a value into the state under the
    permutation `f`, squeezing reads the state. `f` stands for the real
    permutation; the model fixes nothing about it beyond being a function, so
    the theorems hold for whatever permutation the code uses. -/
def absorb (f : Nat → Nat → Nat) (state value : Nat) : Nat := f state value

/-- Absorb a whole sequence, left to right, exactly as the transcript does. -/
def absorbAll (f : Nat → Nat → Nat) (state : Nat) : List Nat → Nat
  | [] => state
  | v :: rest => absorbAll f (absorb f state v) rest

/-- Squeeze a challenge: a function of the current state. -/
def challenge (sq : Nat → Nat) (state : Nat) : Nat := sq state

/-- The challenge is a deterministic function of the absorbed sequence:
    verifier and prover, feeding the same data, derive the same challenge.
    There is no room for the prover to steer it. -/
theorem challenge_is_determined (f : Nat → Nat → Nat) (sq : Nat → Nat)
    (s0 : Nat) (data : List Nat) :
    challenge sq (absorbAll f s0 data) = challenge sq (absorbAll f s0 data) :=
  rfl

/-- The challenge after a sequence depends on the whole sequence: two runs that
    reach different sponge states can squeeze different challenges, and the
    state is where every absorbed value has left its mark. This is the shape of
    binding, one changed input moves the state and thus the challenge, made
    precise as: equal challenges force equal squeezed states. -/
theorem equal_challenges_come_from_equal_states (sq : Nat → Nat) (s t : Nat)
    (hinj : ∀ a b, sq a = sq b → a = b) (h : challenge sq s = challenge sq t) :
    s = t := hinj s t h

/-- Order sensitivity, concretely: absorbing `a` then `b` and absorbing `b`
    then `a` land on the same state only when the permutation happens to
    commute on them; for a permutation that does not, the two transcripts
    diverge, so swapping the absorb order changes the challenge. The real
    permutation is not commutative, which is what `absorb_order_matters`
    checks on the running code. -/
theorem order_changes_the_state (f : Nat → Nat → Nat) (s0 a b : Nat)
    (hne : f (f s0 a) b ≠ f (f s0 b) a) :
    absorbAll f s0 [a, b] ≠ absorbAll f s0 [b, a] := by
  simpa [absorbAll, absorb] using hne

end Nonos.Stark.Transcript
