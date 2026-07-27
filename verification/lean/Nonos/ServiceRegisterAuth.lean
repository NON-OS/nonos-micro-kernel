/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Service registration admission (`caller_can_register` in
`src/services/registry/auth/caller_can_register.rs`). The old rule granted any
caller with `pid <= 64` the right to register a service, so a capsule spawned
early could squat a name without holding `RegisterService`. The fix removes the
shortcut: kernel context (no current pid) is still allowed, but a caller with a
pid must hold the right. The theorems fix that the old rule had a bypass for
low pids, that the fixed rule denies any pid-bearing caller lacking the right
regardless of pid, and that kernel context stays allowed. This complements the
name and port uniqueness proved in `ServiceRegistry`.
-/

namespace Nonos.ServiceRegisterAuth

/-- The caller: `none` is kernel context, `some pid` is a capsule. -/
abbrev Caller := Option Nat

/-- The old admission rule: kernel is allowed; a capsule is allowed if its pid is
    at most 64 or it holds `RegisterService`. -/
def canRegisterOld (c : Caller) (hasRight : Bool) : Bool :=
  match c with
  | none => true
  | some pid => decide (pid ≤ 64) || hasRight

/-- The fixed admission rule: kernel is allowed; a capsule is allowed only if it
    holds `RegisterService`. -/
def canRegisterNew (c : Caller) (hasRight : Bool) : Bool :=
  match c with
  | none => true
  | some _ => hasRight

/-- Kernel context is allowed under the fixed rule (trusted boot services still
    register). -/
theorem kernel_allowed (hasRight : Bool) : canRegisterNew none hasRight = true := rfl

/-- The bug: under the old rule a capsule with a low pid could register with no
    right at all. -/
theorem old_low_pid_bypass : ∃ pid, canRegisterOld (some pid) false = true :=
  ⟨1, by decide⟩

/-- The fix: under the new rule a pid-bearing caller without the right is denied,
    for every pid, so the low-pid bypass is closed. -/
theorem new_no_bypass (pid : Nat) : canRegisterNew (some pid) false = false := rfl

/-- Admission under the fixed rule requires either kernel context or the right:
    there is no third accepting case. -/
theorem new_requires_right (c : Caller) (hasRight : Bool)
    (h : canRegisterNew c hasRight = true) : c = none ∨ hasRight = true := by
  cases c with
  | none => exact Or.inl rfl
  | some pid => exact Or.inr h

end Nonos.ServiceRegisterAuth
