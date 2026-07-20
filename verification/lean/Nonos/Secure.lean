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
user copies, boot, capsule spawn, and device DMA mapping. `Secure` conjoins
authority conservation (no step manufactures a right nobody held initially),
memory isolation (no installed mapping is writable and executable, and every
accepted user copy lies inside user space), anti-rollback (the version floor
never decreases), attestation (only an attested capsule is admitted), and DMA
confinement (every installed DMA mapping passed the broker's admission guard,
so it is owned by its claiming caller and bounded by the device class's page
limit). `every_trace_is_secure` proves the conjunction holds after every trace
of every length, so a hostile sequence of these operations cannot escape it.

Each transition's obligation is discharged on the real kernel code
elsewhere: the capability steps by verification/verus/src/capabilities.rs
and the kernel_proofs differential harnesses over src/capabilities/bits.rs,
the mapping gate by the to_pte_flags proofs over
src/memory/paging/types/permissions/convert.rs, the copy gate by the
check_range proofs over src/usercopy/policy.rs, the boot step by
nonos-bootloader/boot_proofs, and the DMA-map guard by the broker validate
over src/hardware/broker (modelled in Nonos.DmaMap).
-/

import Nonos.Capability
import Nonos.Isolation
import Nonos.AntiRollback
import Nonos.DmaMap
import Nonos.ElfPhdr
import Nonos.IrqBind
import Nonos.Quota

namespace Nonos.Secure

open Nonos.Capability (Caps Grants)

/-- The inputs the ELF program-header bounds check reads: the image length, the
    header-table offset, the entry size, the entry count, and the size the loader
    expects each entry to be. -/
structure ElfSpec where
  dataLen : Nat
  phoff : Nat
  phsize : Nat
  phnum : Nat
  expected : Nat

/-- The abstract machine state: each process's capability token, the
    installed page permissions, the accepted user-copy log, the anti-rollback
    floor, the admitted-capsule log, the attestation oracle, the installed
    DMA mappings as the request/claim pair the broker admitted, and the ELF
    header tables the loader accepted. -/
structure State where
  token : Nat → Caps
  mapped : Nat → Option Isolation.Perm
  copies : List (Nat × Nat)
  floor : Nat
  admitted : List Nat
  attest : Nat → Bool
  dma : List (DmaMap.Req × DmaMap.Claim)
  elf : List ElfSpec
  irq : List IrqBind.Input
  quota : Nat → Quota.Q

/-- The security-relevant transitions. -/
inductive Syscall where
  | attenuate (pid : Nat) (mask : Caps)
  | transfer (src dst : Nat) (b : Nat)
  | revoke (pid : Nat) (b : Nat)
  | mapPage (page : Nat) (p : Isolation.Perm)
  | userCopy (addr len : Nat)
  | boot (v : Nat)
  | spawn (cap : Nat)
  | dmaMap (r : DmaMap.Req) (c : DmaMap.Claim)
  | loadElf (e : ElfSpec)
  | bindMsix (i : IrqBind.Input)
  | acquire (pid : Nat) (n : Nat)

/-- One step of the machine. Every arm is the guard the kernel enforces:
    attenuation meets the caller's own token, a transfer moves only a right
    the source holds, mapping rejects a W^X violation, a copy is logged only
    if the range check accepts it, spawn admits only an attested capsule, and a
    DMA mapping is installed only when the broker's `validate` returns `ok`. -/
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
  | .spawn cap =>
      if s.attest cap then { s with admitted := cap :: s.admitted } else s
  | .dmaMap r c =>
      if DmaMap.validate r c = .ok then { s with dma := (r, c) :: s.dma } else s
  | .loadElf e =>
      if ElfPhdr.check e.dataLen e.phoff e.phsize e.phnum e.expected
          = .ok e.phoff e.phsize e.phnum then
        { s with elf := e :: s.elf }
      else s
  | .bindMsix i =>
      if IrqBind.validate i = .ok then { s with irq := i :: s.irq } else s
  | .acquire pid n =>
      { s with quota := fun p =>
          if p = pid then Quota.acquire (s.quota p) n else s.quota p }

/-- A whole execution. -/
def run (s : State) : List Syscall → State
  | [] => s
  | sc :: rest => run (step s sc) rest

/-- The conjoined security property, relative to the initial state.

    Authority conservation: any right any process holds now, some process
    held at the start; no sequence of attenuations, transfers, and
    revocations manufactures authority. Isolation: no installed mapping is
    writable and executable, and every byte of every accepted copy lies in
    user space. Anti-rollback: the floor never went down. Attestation: every
    admitted capsule was attested. DMA confinement: every installed DMA
    mapping passed the broker's admission guard. -/
structure Secure (s0 s : State) : Prop where
  authority : ∀ b, (∃ p, Grants (s.token p) b) → ∃ p, Grants (s0.token p) b
  no_wx : ∀ page pm, s.mapped page = some pm →
    ¬(pm.write = true ∧ pm.execute = true)
  copies_in_user_space : ∀ c ∈ s.copies, ∀ i < c.2, c.1 + i ≤ Isolation.userEnd
  floor_monotone : s0.floor ≤ s.floor
  admitted_attested : ∀ c ∈ s.admitted, s.attest c = true
  dma_confined : ∀ rc ∈ s.dma, DmaMap.validate rc.1 rc.2 = .ok
  elf_in_bounds : ∀ e ∈ s.elf,
    ElfPhdr.check e.dataLen e.phoff e.phsize e.phnum e.expected
      = .ok e.phoff e.phsize e.phnum
  irq_confined : ∀ i ∈ s.irq, IrqBind.validate i = .ok
  quota_ok : ∀ p, Quota.ok (s.quota p)

/-- A state with no W^X mapping, no logged copies, no admitted capsule, and no
    DMA mapping is secure against itself: the induction base. -/
theorem secure_refl (s0 : State)
    (hwx : ∀ page pm, s0.mapped page = some pm →
      ¬(pm.write = true ∧ pm.execute = true))
    (hcopies : s0.copies = []) (hadmit : s0.admitted = [])
    (hdma : s0.dma = []) (helf : s0.elf = []) (hirq : s0.irq = [])
    (hquota : ∀ p, Quota.ok (s0.quota p)) :
    Secure s0 s0 := by
  refine ⟨fun b h => h, hwx, ?_, Nat.le_refl _, ?_, ?_, ?_, ?_, hquota⟩
  · intro c hc
    rw [hcopies] at hc
    exact absurd hc (List.not_mem_nil c)
  · intro c hc
    rw [hadmit] at hc
    exact absurd hc (List.not_mem_nil c)
  · intro rc hrc
    rw [hdma] at hrc
    exact absurd hrc (List.not_mem_nil rc)
  · intro e he
    rw [helf] at he
    exact absurd he (List.not_mem_nil e)
  · intro i hi
    rw [hirq] at hi
    exact absurd hi (List.not_mem_nil i)

/-- The heart of the theorem: every single transition preserves the whole
    conjunction. -/
theorem step_preserves_secure (s0 s : State) (sc : Syscall)
    (h : Secure s0 s) : Secure s0 (step s sc) := by
  obtain ⟨hauth, hwx, hcopy, hfloor, hadm, hdma, helf, hirq, hquota⟩ := h
  cases sc with
  | attenuate pid mask =>
    refine ⟨?_, hwx, hcopy, hfloor, hadm, hdma, helf, hirq, hquota⟩
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
      refine ⟨?_, hwx, hcopy, hfloor, hadm, hdma, helf, hirq, hquota⟩
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
    · exact ⟨hauth, hwx, hcopy, hfloor, hadm, hdma, helf, hirq, hquota⟩
  | revoke pid b' =>
    refine ⟨?_, hwx, hcopy, hfloor, hadm, hdma, helf, hirq, hquota⟩
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
    · exact ⟨hauth, hwx, hcopy, hfloor, hadm, hdma, helf, hirq, hquota⟩
    · rename_i hnotwx
      refine ⟨hauth, ?_, hcopy, hfloor, hadm, hdma, helf, hirq, hquota⟩
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
      refine ⟨hauth, hwx, ?_, hfloor, hadm, hdma, helf, hirq, hquota⟩
      intro c hc
      simp at hc
      obtain hnew | hold := hc
      · subst hnew
        intro i hi
        obtain ⟨_, _, hbound⟩ := hacc
        omega
      · exact hcopy c hold
    · exact ⟨hauth, hwx, hcopy, hfloor, hadm, hdma, helf, hirq, hquota⟩
  | boot v =>
    refine ⟨hauth, hwx, hcopy, ?_, hadm, hdma, helf, hirq, hquota⟩
    exact Nat.le_trans hfloor
      (AntiRollback.update_never_lowers_floor { floor := s.floor } v)
  | spawn cap =>
    simp only [step]
    by_cases hatt : s.attest cap = true
    · rw [if_pos hatt]
      refine ⟨hauth, hwx, hcopy, hfloor, ?_, hdma, helf, hirq, hquota⟩
      intro c hc
      rcases List.mem_cons.mp hc with hc | hc
      · subst hc; exact hatt
      · exact hadm c hc
    · rw [if_neg hatt]
      exact ⟨hauth, hwx, hcopy, hfloor, hadm, hdma, helf, hirq, hquota⟩
  | dmaMap r c =>
    simp only [step]
    by_cases hv : DmaMap.validate r c = .ok
    · rw [if_pos hv]
      refine ⟨hauth, hwx, hcopy, hfloor, hadm, ?_, helf, hirq, hquota⟩
      intro rc hmem
      rcases List.mem_cons.mp hmem with hhead | htail
      · subst hhead; exact hv
      · exact hdma rc htail
    · rw [if_neg hv]
      exact ⟨hauth, hwx, hcopy, hfloor, hadm, hdma, helf, hirq, hquota⟩
  | loadElf e =>
    simp only [step]
    by_cases hv : ElfPhdr.check e.dataLen e.phoff e.phsize e.phnum e.expected
        = .ok e.phoff e.phsize e.phnum
    · rw [if_pos hv]
      refine ⟨hauth, hwx, hcopy, hfloor, hadm, hdma, ?_, hirq, hquota⟩
      intro e' hmem
      rcases List.mem_cons.mp hmem with hhead | htail
      · subst hhead; exact hv
      · exact helf e' htail
    · rw [if_neg hv]
      exact ⟨hauth, hwx, hcopy, hfloor, hadm, hdma, helf, hirq, hquota⟩
  | bindMsix i =>
    simp only [step]
    by_cases hv : IrqBind.validate i = .ok
    · rw [if_pos hv]
      refine ⟨hauth, hwx, hcopy, hfloor, hadm, hdma, helf, ?_, hquota⟩
      intro i' hmem
      rcases List.mem_cons.mp hmem with hhead | htail
      · subst hhead; exact hv
      · exact hirq i' htail
    · rw [if_neg hv]
      exact ⟨hauth, hwx, hcopy, hfloor, hadm, hdma, helf, hirq, hquota⟩
  | acquire pid n =>
    refine ⟨hauth, hwx, hcopy, hfloor, hadm, hdma, helf, hirq, ?_⟩
    intro p
    simp only [step]
    by_cases hp : p = pid
    · rw [if_pos hp]
      exact Quota.acquire_preserves_ok (s.quota p) n (hquota p)
    · rw [if_neg hp]
      exact hquota p

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
    (hcopies : s0.copies = []) (hadmit : s0.admitted = [])
    (hdma : s0.dma = []) (helf : s0.elf = []) (hirq : s0.irq = [])
    (hquota : ∀ p, Quota.ok (s0.quota p)) :
    Secure s0 (run s0 tr) :=
  run_preserves_secure s0 tr s0
    (secure_refl s0 hwx hcopies hadmit hdma helf hirq hquota)

/-- Corollary of the conjoined invariant (availability): in any reachable state
    every domain's resource use is within its cap, so no capsule can consume past
    its quota and starve the others. -/
theorem quota_within_cap (s0 s : State) (h : Secure s0 s) (p : Nat) :
    (s.quota p).used ≤ (s.quota p).cap :=
  h.quota_ok p

/-- Corollary of the conjoined invariant: in any reachable state, every
    installed DMA mapping is owned by the caller that claimed the device. A
    caller can never program a device it does not hold to DMA on its behalf. -/
theorem dma_owned_by_caller (s0 s : State) (h : Secure s0 s)
    (rc : DmaMap.Req × DmaMap.Claim) (hmem : rc ∈ s.dma) :
    rc.2.pid = rc.1.reqPid :=
  DmaMap.accepted_owned_by_caller rc.1 rc.2 (h.dma_confined rc hmem)

/-- Corollary of the conjoined invariant: every installed DMA mapping is within
    its device class's page limit, so no reachable state has an unbounded DMA
    window. -/
theorem dma_within_class_limit (s0 s : State) (h : Secure s0 s)
    (rc : DmaMap.Req × DmaMap.Claim) (hmem : rc ∈ s.dma) :
    rc.1.length / DmaMap.page ≤ rc.2.pageLimit :=
  DmaMap.accepted_within_class_limit rc.1 rc.2 (h.dma_confined rc hmem)

/-- Corollary of the conjoined invariant: in any reachable state, every accepted
    non-empty ELF program-header table lies wholly inside its image, so the loader
    never reads a header entry past the buffer end. -/
theorem elf_table_in_bounds (s0 s : State) (h : Secure s0 s)
    (e : ElfSpec) (hmem : e ∈ s.elf) (hn : e.phnum ≠ 0) :
    e.phoff + e.phsize * e.phnum ≤ e.dataLen :=
  ElfPhdr.accepted_table_in_bounds e.dataLen e.phoff e.phsize e.phnum e.expected hn
    (h.elf_in_bounds e hmem)

/-- Corollary of the conjoined invariant: every installed MSI-X bind programs a
    vector count within both the broker pool and the device's MSI-X table, so no
    reachable state over-programs an interrupt table. -/
theorem irq_vector_count_bounded (s0 s : State) (h : Secure s0 s)
    (i : IrqBind.Input) (hmem : i ∈ s.irq) :
    i.vectorCount ≠ 0 ∧ i.vectorCount ≤ i.poolCapacity ∧
      i.vectorCount ≤ i.msixTableSize :=
  IrqBind.accepted_vector_count_bounded i (h.irq_confined i hmem)

/-- Corollary of the conjoined invariant: no installed MSI-X bind overwrote an
    existing grant, so a bind is all-or-nothing per device per pid in every
    reachable state. -/
theorem irq_not_already_bound (s0 s : State) (h : Secure s0 s)
    (i : IrqBind.Input) (hmem : i ∈ s.irq) : i.hasExistingGrant = false :=
  IrqBind.accepted_not_already_bound i (h.irq_confined i hmem)

end Nonos.Secure
