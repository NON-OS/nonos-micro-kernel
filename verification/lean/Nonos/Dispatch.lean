/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Syscall dispatch. The registry maps a syscall number to the capability it
requires; an unregistered number has none. Dispatch services a call only if it
is registered and the caller's token grants the required capability. The
theorems below show an unregistered syscall is always denied, a serviced
syscall's capability is always held, and dispatch is monotone in the token ,
there is no path to acceptance that bypasses the capability check.
-/

namespace Nonos.Dispatch

/-- The registry: each syscall number maps to the capability it requires, or
    `none` when the number is not a registered syscall. -/
def Registry := Nat → Option Nat

/-- A caller's token as a capability-membership function. -/
def Token := Nat → Bool

/-- Dispatch: service the call iff it is registered and the token grants the
    required capability. -/
def dispatch (reg : Registry) (tok : Token) (nr : Nat) : Bool :=
  match reg nr with
  | none => false
  | some cap => tok cap

/-- An unregistered syscall is always denied. -/
theorem unregistered_denied (reg : Registry) (tok : Token) (nr : Nat)
    (h : reg nr = none) : dispatch reg tok nr = false := by
  unfold dispatch; rw [h]

/-- A serviced syscall's required capability is held: no gated syscall runs
    without its capability. -/
theorem serviced_requires_cap (reg : Registry) (tok : Token) (nr cap : Nat)
    (hreg : reg nr = some cap) (h : dispatch reg tok nr = true) : tok cap = true := by
  unfold dispatch at h; rw [hreg] at h; exact h

/-- Denying the required capability denies the syscall. -/
theorem no_cap_denied (reg : Registry) (tok : Token) (nr cap : Nat)
    (hreg : reg nr = some cap) (hno : tok cap = false) :
    dispatch reg tok nr = false := by
  unfold dispatch; rw [hreg]; exact hno

/-- `t` subsumes `u` when it grants at least everything `u` grants. -/
def Subsumes (t u : Token) : Prop := ∀ c, u c = true → t c = true

/-- Dispatch is monotone in the token: a call a smaller token could make, a
    larger one can too, but never the reverse, so authority only ever narrows
    down a delegation chain. -/
theorem dispatch_monotone (reg : Registry) (t u : Token) (nr : Nat)
    (hsub : Subsumes t u) (h : dispatch reg u nr = true) : dispatch reg t nr = true := by
  unfold dispatch at h ⊢
  revert h
  cases reg nr with
  | none => intro h; exact h
  | some cap => intro h; exact hsub cap h

/-- Contrapositive: a call the full token cannot make, no attenuation of it can
    make either. -/
theorem denied_below (reg : Registry) (t u : Token) (nr : Nat)
    (hsub : Subsumes t u) (hden : dispatch reg t nr = false) :
    dispatch reg u nr = false := by
  cases hu : dispatch reg u nr with
  | false => rfl
  | true =>
    have hmt := dispatch_monotone reg t u nr hsub hu
    rw [hden] at hmt
    simp at hmt

end Nonos.Dispatch
