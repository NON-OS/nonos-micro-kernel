/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification-level proof of the syscall authorization gate. The real kernel
`is_allowed` cap-table and token-grant logic that implement it are proven to
match by the runnable harnesses in `userland/kernel_proofs` (empty token denied
every syscall; a syscall allowed only for a token that grants the required
capability; monotonicity). Lean proves the property; the code proofs discharge
it on the implementation.
-/

namespace Nonos.Authorization

/-- A capability token as its membership function over capability ids. -/
def Token := Nat → Bool

/-- The token grants capability `cap`. -/
def Grants (t : Token) (cap : Nat) : Prop := t cap = true

def empty : Token := fun _ => false

/-- A syscall guarded by capability `required` is permitted only when the token
    grants that capability. -/
def Allowed (t : Token) (required : Nat) : Prop := Grants t required

/-- The empty token is denied every guarded syscall. -/
theorem empty_token_denied (required : Nat) : ¬ Allowed empty required := by
  unfold Allowed Grants empty; simp

/-- Being allowed implies the required capability is actually held. -/
theorem allowed_requires_capability (t : Token) (required : Nat)
    (h : Allowed t required) : Grants t required := h

/-- `t` subsumes `u` when it grants at least everything `u` grants. -/
def Subsumes (t u : Token) : Prop := ∀ c, u c = true → t c = true

/-- Monotonicity: granting more capabilities never removes access. -/
theorem allow_monotone (t u : Token) (required : Nat)
    (hsub : Subsumes t u) (h : Allowed u required) : Allowed t required := by
  unfold Allowed Grants at *
  exact hsub required h

/-- The gate is exactly a capability check: there is no path to acceptance that
    does not go through holding the required capability. -/
theorem allowed_iff_grants (t : Token) (required : Nat) :
    Allowed t required ↔ Grants t required := Iff.rfl

/-- Two syscalls guarded by the same capability are permitted together or denied
    together: authorization depends only on the guarding capability. -/
theorem same_guard_same_decision (t : Token) (r1 r2 : Nat) (h : r1 = r2) :
    Allowed t r1 ↔ Allowed t r2 := by rw [h]

/-- Subsumption is reflexive: every token delegates to itself. -/
theorem subsumes_refl (t : Token) : Subsumes t t := fun _ h => h

/-- Subsumption is transitive: delegation chains compose, so a token attenuated
    through any number of hands still sits below the original. -/
theorem subsumes_trans (t u v : Token)
    (htu : Subsumes t u) (huv : Subsumes u v) : Subsumes t v :=
  fun c h => htu c (huv c h)

/-- Every token subsumes the empty token: the bottom of the delegation order. -/
theorem subsumes_empty (t : Token) : Subsumes t empty := by
  intro c h
  simp [empty] at h

/-- The security-critical contrapositive of monotonicity: a syscall denied to a
    token is denied to everything below it. No chain of attenuations can reach
    a syscall the original token could not. -/
theorem denied_below (t u : Token) (required : Nat)
    (hsub : Subsumes t u) (hdenied : ¬ Allowed t required) :
    ¬ Allowed u required := by
  intro h
  exact hdenied (allow_monotone t u required hsub h)

end Nonos.Authorization
