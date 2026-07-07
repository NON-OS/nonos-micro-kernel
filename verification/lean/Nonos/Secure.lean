/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The conjoined security theorem. The other modules prove local lemmas about
one operation at a time; this module composes them into a single invariant
over a transition system whose steps are the security-relevant kernel
operations: capability attenuation, transfer, and revocation, page mapping,
user copies, and boot. `Secure` conjoins authority conservation (no step
manufactures a right nobody held initially), memory isolation (no installed
mapping is writable and executable, and every accepted user copy lies inside
user space), and anti-rollback (the version floor never decreases).
`every_trace_is_secure` proves the conjunction holds after every trace of
every length, so a hostile sequence of these operations cannot escape it.

Each transition's obligation is discharged on the real kernel code
elsewhere: the capability steps by verification/verus/src/capabilities.rs
and the kernel_proofs differential harnesses over src/capabilities/bits.rs,
the mapping gate by the to_pte_flags proofs over
src/memory/paging/types/permissions/convert.rs, the copy gate by the
check_range proofs over src/usercopy/policy.rs, and the boot step by
nonos-bootloader/boot_proofs.
-/

import Nonos.Capability
import Nonos.Isolation
import Nonos.AntiRollback

namespace Nonos.Secure

open Nonos.Capability (Caps Grants)

/-- The abstract machine state: each process's capability token, the
    installed page permissions, the accepted user-copy log, and the
    anti-rollback floor. -/
structure State where
  token : Nat → Caps
  mapped : Nat → Option Isolation.Perm
  copies : List (Nat × Nat)
  floor : Nat

/-- The security-relevant transitions. -/
inductive Syscall where
  | attenuate (pid : Nat) (mask : Caps)
  | transfer (src dst : Nat) (b : Nat)
  | revoke (pid : Nat) (b : Nat)
  | mapPage (page : Nat) (p : Isolation.Perm)
  | userCopy (addr len : Nat)
  | boot (v : Nat)

/-- One step of the machine. Every arm is the guard the kernel enforces:
    attenuation meets the caller's own token, a transfer moves only a right
    the source holds, mapping rejects a W^X violation, a copy is logged only
    if the range check accepts it, and boot goes through the anti-rollback
    update. -/
def step (s : State) : Syscall → State
  | .attenuate pid mask =>
      { s with token := fun p =>
          if p = pid then Nonos.Capability.attenuate (s.token p) mask else s.token p }
  | .transfer src dst b =>
      if (s.token src) b then
        { s with token := fun p =>
            if p = dst then Nonos.Capability.grant (s.token p) b else s.token p }
      else s
  | .revoke pid b =>
      { s with token := fun p =>
          if p = pid then Nonos.Capability.revoke (s.token p) b else s.token p }
  | .mapPage page p =>
      if p.write = true ∧ p.execute = true then s
      else { s with mapped := fun q => if q = page then some p else s.mapped q }
  | .userCopy addr len =>
      if addr ≠ 0 ∧ len ≤ Isolation.maxCopy ∧ addr + len ≤ Isolation.userEnd + 1 then
        { s with copies := (addr, len) :: s.copies }
      else s
  | .boot v =>
      { s with floor := (AntiRollback.update { floor := s.floor } v).floor }

/-- A whole execution. -/
def run (s : State) : List Syscall → State
  | [] => s
  | sc :: rest => run (step s sc) rest

/-- The conjoined security property, relative to the initial state.

    Authority conservation: any right any process holds now, some process
    held at the start; no sequence of attenuations, transfers, and
    revocations manufactures authority. Isolation: no installed mapping is
    writable and executable, and every byte of every accepted copy lies in
    user space. Anti-rollback: the floor never went down. -/
structure Secure (s0 s : State) : Prop where
  authority : ∀ b, (∃ p, Grants (s.token p) b) → ∃ p, Grants (s0.token p) b
  no_wx : ∀ page pm, s.mapped page = some pm →
    ¬(pm.write = true ∧ pm.execute = true)
  copies_in_user_space : ∀ c ∈ s.copies, ∀ i < c.2, c.1 + i ≤ Isolation.userEnd
  floor_monotone : s0.floor ≤ s.floor

/-- A state with no W^X mapping and no logged copies is secure against
    itself: the induction base. -/
theorem secure_refl (s0 : State)
    (hwx : ∀ page pm, s0.mapped page = some pm →
      ¬(pm.write = true ∧ pm.execute = true))
    (hcopies : s0.copies = []) : Secure s0 s0 := by
  refine ⟨fun b h => h, hwx, ?_, Nat.le_refl _⟩
  intro c hc
  rw [hcopies] at hc
  exact absurd hc (List.not_mem_nil c)

/-- The heart of the theorem: every single transition preserves the whole
    conjunction. -/
theorem step_preserves_secure (s0 s : State) (sc : Syscall)
    (h : Secure s0 s) : Secure s0 (step s sc) := by
  obtain ⟨hauth, hwx, hcopy, hfloor⟩ := h
  cases sc with
  | attenuate pid mask =>
    refine ⟨?_, hwx, hcopy, hfloor⟩
    intro b ⟨p, hp⟩
    by_cases hpid : p = pid
    · subst hpid
      simp [step, if_pos rfl] at hp
      exact hauth b ⟨p, Nonos.Capability.attenuate_confines _ _ _ hp⟩
    · simp [step, hpid] at hp
      exact hauth b ⟨p, hp⟩
  | transfer src dst b' =>
    simp only [step]
    split
    · rename_i hsrc
      refine ⟨?_, hwx, hcopy, hfloor⟩
      intro b ⟨p, hp⟩
      by_cases hpd : p = dst
      · subst hpd
        have hp' : ((s.token p) b || (b == b')) = true := by
          have h1 : Grants (Nonos.Capability.grant (s.token p) b') b := by simpa using hp
          unfold Nonos.Capability.Grants Nonos.Capability.grant at h1
          exact h1
        simp only [Bool.or_eq_true] at hp'
        obtain hold | hnew := hp'
        · exact hauth b ⟨p, hold⟩
        · have hb : b = b' := by simpa using hnew
          subst hb
          exact hauth b ⟨src, hsrc⟩
      · have hp' : (s.token p) b = true := by simpa [hpd] using hp
        exact hauth b ⟨p, hp'⟩
    · exact ⟨hauth, hwx, hcopy, hfloor⟩
  | revoke pid b' =>
    refine ⟨?_, hwx, hcopy, hfloor⟩
    intro b ⟨p, hp⟩
    by_cases hpid : p = pid
    · subst hpid
      simp [step, if_pos rfl] at hp
      exact hauth b ⟨p, Nonos.Capability.revoke_leq _ _ _ hp⟩
    · simp [step, hpid] at hp
      exact hauth b ⟨p, hp⟩
  | mapPage page p =>
    simp only [step]
    split
    · exact ⟨hauth, hwx, hcopy, hfloor⟩
    · rename_i hnotwx
      refine ⟨hauth, ?_, hcopy, hfloor⟩
      intro q pm hq
      by_cases hqp : q = page
      · subst hqp
        simp at hq
        subst hq
        exact hnotwx
      · simp [hqp] at hq
        exact hwx q pm hq
  | userCopy addr len =>
    simp only [step]
    split
    · rename_i hacc
      refine ⟨hauth, hwx, ?_, hfloor⟩
      intro c hc
      simp at hc
      obtain hnew | hold := hc
      · subst hnew
        intro i hi
        obtain ⟨_, _, hbound⟩ := hacc
        omega
      · exact hcopy c hold
    · exact ⟨hauth, hwx, hcopy, hfloor⟩
  | boot v =>
    refine ⟨hauth, hwx, hcopy, ?_⟩
    exact Nat.le_trans hfloor
      (AntiRollback.update_never_lowers_floor { floor := s.floor } v)

/-- Security is preserved along any run, from any secure intermediate
    state. -/
theorem run_preserves_secure (s0 : State) (tr : List Syscall) (s : State)
    (h : Secure s0 s) : Secure s0 (run s tr) := by
  induction tr generalizing s with
  | nil => exact h
  | cons sc rest ih => exact ih (step s sc) (step_preserves_secure s0 s sc h)

/-- The conjoined security theorem: after any trace of any length, from any
    clean start, the machine is still secure. The attacker chooses every
    transition and never escapes the conjunction. -/
theorem every_trace_is_secure (s0 : State) (tr : List Syscall)
    (hwx : ∀ page pm, s0.mapped page = some pm →
      ¬(pm.write = true ∧ pm.execute = true))
    (hcopies : s0.copies = []) : Secure s0 (run s0 tr) :=
  run_preserves_secure s0 tr s0 (secure_refl s0 hwx hcopies)

end Nonos.Secure
