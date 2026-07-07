/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification of the STARK base field arithmetic. The kernel's transparent
proof system computes in the Goldilocks field, integers modulo
p = 2^64 - 2^32 + 1, with every element kept as its canonical representative.
This module proves the ring laws of modular arithmetic for every modulus, so
they hold for Goldilocks in particular: closure into the canonical range,
associativity and commutativity of addition and multiplication, the
identities, additive inverses, and distributivity. `userland/stark_proofs`
discharges the same laws on the real `src/crypto/stark/field` code in
`field_tests.rs`: `addition_is_an_abelian_group`,
`multiplication_is_a_commutative_monoid_and_distributes`, and
`canonical_representatives_and_edge_values`.

The one field law this module deliberately does not state is the existence of
multiplicative inverses, which depends on p being prime; the code computes
x^(p-2) and `every_nonzero_element_has_an_inverse` checks the defining
equation on the real field, edge values included. Stating it abstractly would
need a primality development out of proportion to what it adds.
-/

namespace Nonos.Stark.Field

/-- Addition on canonical representatives. -/
def add (p a b : Nat) : Nat := (a + b) % p

/-- The additive inverse of a canonical representative. -/
def neg (p a : Nat) : Nat := (p - a % p) % p

/-- Multiplication on canonical representatives. -/
def mul (p a b : Nat) : Nat := (a * b) % p

private theorem mod_mod (x p : Nat) : x % p % p = x % p :=
  Nat.mod_mod_of_dvd x (Nat.dvd_refl p)

private theorem add_reduce_left (p x y : Nat) : (x % p + y) % p = (x + y) % p := by
  rw [Nat.add_mod, mod_mod, ← Nat.add_mod]

private theorem add_reduce_right (p x y : Nat) : (x + y % p) % p = (x + y) % p := by
  rw [Nat.add_comm x (y % p), add_reduce_left, Nat.add_comm]

private theorem mul_reduce_left (p x y : Nat) : (x % p * y) % p = (x * y) % p := by
  rw [Nat.mul_mod, mod_mod, ← Nat.mul_mod]

private theorem mul_reduce_right (p x y : Nat) : (x * (y % p)) % p = (x * y) % p := by
  rw [Nat.mul_comm x (y % p), mul_reduce_left, Nat.mul_comm]

/-- Every sum and product lands back in the canonical range: the
    representation is closed under the field operations. -/
theorem results_are_canonical (p a b : Nat) (hp : 0 < p) :
    add p a b < p ∧ mul p a b < p :=
  ⟨Nat.mod_lt _ hp, Nat.mod_lt _ hp⟩

/-- Addition is commutative. -/
theorem add_is_commutative (p a b : Nat) : add p a b = add p b a := by
  unfold add
  rw [Nat.add_comm]

/-- Addition is associative on representatives: reducing between steps
    changes nothing. -/
theorem add_is_associative (p a b c : Nat) :
    add p (add p a b) c = add p a (add p b c) := by
  unfold add
  rw [add_reduce_left, add_reduce_right, Nat.add_assoc]

/-- Zero is the additive identity on canonical representatives. -/
theorem zero_is_the_additive_identity (p a : Nat) (ha : a < p) :
    add p a 0 = a := by
  unfold add
  rw [Nat.add_zero]
  exact Nat.mod_eq_of_lt ha

/-- Every canonical representative has an additive inverse: the group law
    that completes the abelian group of addition. -/
theorem every_element_has_an_additive_inverse (p a : Nat)
    (ha : a < p) : add p a (neg p a) = 0 := by
  unfold add neg
  rw [Nat.mod_eq_of_lt ha]
  by_cases h0 : a = 0
  · subst h0
    simp [Nat.mod_self]
  · have h2 : (p - a) % p = p - a := Nat.mod_eq_of_lt (by omega)
    rw [h2]
    have h3 : a + (p - a) = p := by omega
    rw [h3, Nat.mod_self]

/-- Multiplication is commutative. -/
theorem mul_is_commutative (p a b : Nat) : mul p a b = mul p b a := by
  unfold mul
  rw [Nat.mul_comm]

/-- Multiplication is associative on representatives. -/
theorem mul_is_associative (p a b c : Nat) :
    mul p (mul p a b) c = mul p a (mul p b c) := by
  unfold mul
  rw [mul_reduce_left, mul_reduce_right, Nat.mul_assoc]

/-- One is the multiplicative identity on canonical representatives. -/
theorem one_is_the_multiplicative_identity (p a : Nat) (ha : a < p) :
    mul p a 1 = a := by
  unfold mul
  rw [Nat.mul_one]
  exact Nat.mod_eq_of_lt ha

/-- Multiplication distributes over addition: the law that makes the
    structure a ring, and the one every polynomial identity in the proof
    system leans on. -/
theorem mul_distributes_over_add (p a b c : Nat) :
    mul p a (add p b c) = add p (mul p a b) (mul p a c) := by
  unfold mul add
  rw [mul_reduce_right, add_reduce_left, add_reduce_right, Nat.left_distrib]

end Nonos.Stark.Field
