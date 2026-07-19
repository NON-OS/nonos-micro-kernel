/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Authority non-interference over the conjoined security machine of `Nonos.Secure`.

The conjoined theorem proves the whole system stays inside its safety envelope
under any adversarial trace. This module proves a complementary, and stronger in
flavour, property: a capsule's authority is both untouched by, and invisible to,
the operations it is not part of. A transition can change a domain's capability
token only if it names that domain as the process whose authority changes; every
other transition, whatever else it does to memory, DMA, the loader or the
interrupt table, leaves that token exactly as it was. From this locality the
non-interference statement follows: two system states that agree on a domain's
authority still agree after any two traces that never name it, so nothing another
domain does can be observed in, or leak into, that domain's authority.

This is the integrity backbone of the design. It is the kernel-side analogue of
the classical non-interference guarantee, discharged here by machine-checked
case analysis over the real transition relation rather than argued informally.
-/

import Nonos.Secure

namespace Nonos.NonInterference

open Nonos.Secure

/-! ### The unwinding framework

Non-interference is proved here through an unwinding structure, the standard
device for such results: an observation `view` of the state and a predicate
`touches` marking the transitions that may change it, together with the single
local obligation that a non-touching transition preserves the observation. From
those local data the global locality and non-interference theorems follow once
and for all, and every concrete dimension of the machine (authority, memory, the
admitted-capsule set, the DMA log) is then an instance rather than a fresh
induction. -/

/-- An unwinding datum: an observation of the state, the transitions that may
    change it, and the local frame condition that ties them together. -/
structure Unwinding (V : Type) where
  view : State → V
  touches : Syscall → Prop
  step_frame : ∀ s sc, ¬ touches sc → view (step s sc) = view s

/-- Locality, once for all views: along any trace whose transitions never touch
    the observation, the observation is exactly its initial value. -/
theorem Unwinding.locality {V : Type} (u : Unwinding V) (s : State)
    (tr : List Syscall) (h : ∀ sc ∈ tr, ¬ u.touches sc) :
    u.view (run s tr) = u.view s := by
  induction tr generalizing s with
  | nil => rfl
  | cons sc rest ih =>
    have hrest : ∀ x ∈ rest, ¬ u.touches x :=
      fun x hx => h x (List.mem_cons_of_mem sc hx)
    calc u.view (run s (sc :: rest))
        = u.view (run (step s sc) rest) := rfl
      _ = u.view (step s sc) := ih (step s sc) hrest
      _ = u.view s := u.step_frame s sc (h sc (List.mem_cons_self sc rest))

/-- Non-interference, once for all views: two states with the same observation
    keep the same observation after any two traces that never touch it. -/
theorem Unwinding.noninterference {V : Type} (u : Unwinding V) (s s' : State)
    (tr tr' : List Syscall) (hagree : u.view s = u.view s')
    (h : ∀ sc ∈ tr, ¬ u.touches sc) (h' : ∀ sc ∈ tr', ¬ u.touches sc) :
    u.view (run s tr) = u.view (run s' tr') := by
  rw [u.locality s tr h, u.locality s' tr' h', hagree]

/-- A transition can modify domain `q`'s capability token only if it names `q` as
    the process whose authority changes: an attenuation or a revocation of `q`,
    or a transfer whose destination is `q`. Every other transition leaves every
    token untouched. -/
def TouchesToken (q : Nat) : Syscall → Prop
  | .attenuate pid _ => pid = q
  | .transfer _ dst _ => dst = q
  | .revoke pid _ => pid = q
  | _ => False

/-- A single transition that does not name `q` leaves `q`'s token unchanged, no
    matter what else it does. This is the per-step frame lemma. -/
theorem step_preserves_token (q : Nat) (s : State) (sc : Syscall)
    (h : ¬ TouchesToken q sc) : (step s sc).token q = s.token q := by
  cases sc with
  | attenuate pid mask =>
    have hpq : ¬ (q = pid) := fun he => h he.symm
    simp only [step]
    rw [if_neg hpq]
  | transfer src dst b =>
    have hdq : ¬ (q = dst) := fun he => h he.symm
    simp only [step]
    split
    · show (if q = dst then Nonos.Capability.grant (s.token q) b else s.token q)
          = s.token q
      rw [if_neg hdq]
    · rfl
  | revoke pid b =>
    have hpq : ¬ (q = pid) := fun he => h he.symm
    simp only [step]
    rw [if_neg hpq]
  | mapPage page p => simp only [step]; split <;> rfl
  | userCopy addr len => simp only [step]; split <;> rfl
  | boot v => rfl
  | spawn cap => simp only [step]; split <;> rfl
  | dmaMap r c => simp only [step]; split <;> rfl
  | loadElf e => simp only [step]; split <;> rfl
  | bindMsix i => simp only [step]; split <;> rfl
  | acquire pid n => rfl

/-- Token locality: along any trace whose transitions never name `q`, domain
    `q`'s token is exactly the one it started with. Authority is not changed by
    anything the domain is not part of. -/
theorem token_locality (q : Nat) (s : State) (tr : List Syscall)
    (h : ∀ sc ∈ tr, ¬ TouchesToken q sc) : (run s tr).token q = s.token q := by
  induction tr generalizing s with
  | nil => rfl
  | cons sc rest ih =>
    have hrest : ∀ x ∈ rest, ¬ TouchesToken q x :=
      fun x hx => h x (List.mem_cons_of_mem sc hx)
    have hhead : ¬ TouchesToken q sc := h sc (List.mem_cons_self sc rest)
    calc (run s (sc :: rest)).token q
        = (run (step s sc) rest).token q := rfl
      _ = (step s sc).token q := ih (step s sc) hrest
      _ = s.token q := step_preserves_token q s sc hhead

/-- Authority non-interference: two states that agree on domain `q`'s token still
    agree on it after any two traces that never name `q`. Whatever the rest of the
    system does, and however the adversary interleaves it, it cannot be observed
    in, or leak into, `q`'s authority. -/
theorem token_noninterference (q : Nat) (s s' : State) (tr tr' : List Syscall)
    (hagree : s.token q = s'.token q)
    (h : ∀ sc ∈ tr, ¬ TouchesToken q sc)
    (h' : ∀ sc ∈ tr', ¬ TouchesToken q sc) :
    (run s tr).token q = (run s' tr').token q := by
  rw [token_locality q s tr h, token_locality q s' tr' h', hagree]

/-- A grant a domain does not hold cannot appear in its token through operations
    that do not name it: the read-side reading of non-interference. -/
theorem no_authority_leak (q : Nat) (s : State) (tr : List Syscall) (b : Nat)
    (h : ∀ sc ∈ tr, ¬ TouchesToken q sc)
    (hno : (s.token q) b = false) : (run s tr).token q b = false := by
  rw [token_locality q s tr h]; exact hno

/-- A transition can add a right to `q` only by transferring one to it: a
    transfer whose destination is `q`. Attenuation and revocation only remove
    rights, and no other transition touches a token. -/
def GrantsTo (q : Nat) : Syscall → Prop
  | .transfer _ dst _ => dst = q
  | _ => False

/-- A single non-granting transition never adds a right to `q`: if `q` did not
    hold `b` before, it does not hold it after. This is the authorized-flow
    analogue of the frame lemma: the only permitted increase in authority is an
    explicit transfer, so every other step preserves the absence of a right. -/
theorem step_no_gain (q : Nat) (s : State) (sc : Syscall) (b : Nat)
    (h : ¬ GrantsTo q sc) (hb : s.token q b = false) :
    (step s sc).token q b = false := by
  cases sc with
  | attenuate pid mask =>
    simp only [step]
    by_cases hqp : q = pid
    · rw [if_pos hqp]; simp [Nonos.Capability.attenuate, hb]
    · rw [if_neg hqp]; exact hb
  | transfer src dst b' =>
    have hdq : ¬ (q = dst) := fun he => h he.symm
    simp only [step]
    split
    · show (if q = dst then Nonos.Capability.grant (s.token q) b' else s.token q) b = false
      rw [if_neg hdq]; exact hb
    · exact hb
  | revoke pid b' =>
    simp only [step]
    by_cases hqp : q = pid
    · rw [if_pos hqp]; simp [Nonos.Capability.revoke, hb]
    · rw [if_neg hqp]; exact hb
  | mapPage page p => simp only [step]; split <;> exact hb
  | userCopy addr len => simp only [step]; split <;> exact hb
  | boot v => exact hb
  | spawn cap => simp only [step]; split <;> exact hb
  | dmaMap r c => simp only [step]; split <;> exact hb
  | loadElf e => simp only [step]; split <;> exact hb
  | bindMsix i => simp only [step]; split <;> exact hb
  | acquire pid n => exact hb

/-- No authority amplification: along any trace with no transfer naming `q`, a
    right `q` did not hold at the start it never comes to hold. Authority does
    not grow except through an explicit, authorized transfer, whatever else the
    rest of the system does. This is the intransitive, authorized-flow companion
    to non-interference. -/
theorem no_authority_amplification (q : Nat) (b : Nat) :
    ∀ (tr : List Syscall) (s : State),
      (∀ sc ∈ tr, ¬ GrantsTo q sc) → s.token q b = false →
        (run s tr).token q b = false
  | [], _, _, hb => hb
  | sc :: rest, s, h, hb =>
      no_authority_amplification q b rest (step s sc)
        (fun x hx => h x (List.mem_cons_of_mem sc hx))
        (step_no_gain q s sc b (h sc (List.mem_cons_self sc rest)) hb)

/-! ### The memory dimension

The lemmas above isolate a domain's *authority*. The same non-interference holds
of the *memory image* at page granularity: a page's installed permission is
changed only by a mapping that names that page, so one page's mapping is
untouched by, and invisible to, operations on any other page. Together the two
dimensions give a non-interference result over both the authority and the memory
components of the machine state. -/

/-- A transition can change the permission installed at page `pg` only if it maps
    that page: a `mapPage pg` transition. Every other transition leaves that
    page's mapping exactly as it was. -/
def TouchesPage (pg : Nat) : Syscall → Prop
  | .mapPage page _ => page = pg
  | _ => False

/-- The per-step frame lemma for memory: a transition that does not map page `pg`
    leaves its installed permission unchanged. -/
theorem step_preserves_mapping (pg : Nat) (s : State) (sc : Syscall)
    (h : ¬ TouchesPage pg sc) : (step s sc).mapped pg = s.mapped pg := by
  cases sc with
  | attenuate pid mask => rfl
  | transfer src dst b => simp only [step]; split <;> rfl
  | revoke pid b => rfl
  | mapPage page p =>
    have hpg : ¬ (pg = page) := fun he => h he.symm
    simp only [step]
    split
    · rfl
    · show (if pg = page then some p else s.mapped pg) = s.mapped pg
      rw [if_neg hpg]
  | userCopy addr len => simp only [step]; split <;> rfl
  | boot v => rfl
  | spawn cap => simp only [step]; split <;> rfl
  | dmaMap r c => simp only [step]; split <;> rfl
  | loadElf e => simp only [step]; split <;> rfl
  | bindMsix i => simp only [step]; split <;> rfl
  | acquire pid n => rfl

/-- Memory locality: over any trace whose transitions never map page `pg`, that
    page's installed permission is exactly the one it started with. -/
theorem mapping_locality (pg : Nat) (s : State) (tr : List Syscall)
    (h : ∀ sc ∈ tr, ¬ TouchesPage pg sc) : (run s tr).mapped pg = s.mapped pg := by
  induction tr generalizing s with
  | nil => rfl
  | cons sc rest ih =>
    have hrest : ∀ x ∈ rest, ¬ TouchesPage pg x :=
      fun x hx => h x (List.mem_cons_of_mem sc hx)
    calc (run s (sc :: rest)).mapped pg
        = (run (step s sc) rest).mapped pg := rfl
      _ = (step s sc).mapped pg := ih (step s sc) hrest
      _ = s.mapped pg := step_preserves_mapping pg s sc (h sc (List.mem_cons_self sc rest))

/-- Memory non-interference: two states that agree on page `pg`'s mapping still
    agree after any two traces that never map `pg`. What happens to the rest of
    the address space cannot be observed in, or leak into, that page. -/
theorem mapping_noninterference (pg : Nat) (s s' : State) (tr tr' : List Syscall)
    (hagree : s.mapped pg = s'.mapped pg)
    (h : ∀ sc ∈ tr, ¬ TouchesPage pg sc)
    (h' : ∀ sc ∈ tr', ¬ TouchesPage pg sc) :
    (run s tr).mapped pg = (run s' tr').mapped pg := by
  rw [mapping_locality pg s tr h, mapping_locality pg s' tr' h', hagree]

/-! ### The dimensions as unwinding instances

The authority and memory frame lemmas above are exactly the local obligation the
unwinding framework asks for, so each dimension is an instance. Two further
dimensions, the admitted-capsule set and the DMA log, are added the same way with
only their one-line frame lemma, which is what makes the framework pay off. -/

/-- Authority as an unwinding datum. -/
def tokenUnwinding (q : Nat) : Unwinding (Nat → Bool) where
  view := fun s => s.token q
  touches := TouchesToken q
  step_frame := step_preserves_token q

/-- Memory as an unwinding datum. -/
def pageUnwinding (pg : Nat) : Unwinding (Option Isolation.Perm) where
  view := fun s => s.mapped pg
  touches := TouchesPage pg
  step_frame := step_preserves_mapping pg

/-- Only a `spawn` transition changes the admitted-capsule set. -/
def TouchesAdmitted : Syscall → Prop
  | .spawn _ => True
  | _ => False

theorem step_preserves_admitted (s : State) (sc : Syscall)
    (h : ¬ TouchesAdmitted sc) : (step s sc).admitted = s.admitted := by
  cases sc with
  | spawn cap => exact absurd trivial h
  | attenuate pid mask => rfl
  | transfer src dst b => simp only [step]; split <;> rfl
  | revoke pid b => rfl
  | mapPage page p => simp only [step]; split <;> rfl
  | userCopy addr len => simp only [step]; split <;> rfl
  | boot v => rfl
  | dmaMap r c => simp only [step]; split <;> rfl
  | loadElf e => simp only [step]; split <;> rfl
  | bindMsix i => simp only [step]; split <;> rfl
  | acquire pid n => rfl

/-- The admitted-capsule set as an unwinding datum. -/
def admittedUnwinding : Unwinding (List Nat) where
  view := fun s => s.admitted
  touches := TouchesAdmitted
  step_frame := step_preserves_admitted

/-- Only a `dmaMap` transition changes the DMA log. -/
def TouchesDma : Syscall → Prop
  | .dmaMap _ _ => True
  | _ => False

theorem step_preserves_dma (s : State) (sc : Syscall)
    (h : ¬ TouchesDma sc) : (step s sc).dma = s.dma := by
  cases sc with
  | dmaMap r c => exact absurd trivial h
  | attenuate pid mask => rfl
  | transfer src dst b => simp only [step]; split <;> rfl
  | revoke pid b => rfl
  | mapPage page p => simp only [step]; split <;> rfl
  | userCopy addr len => simp only [step]; split <;> rfl
  | boot v => rfl
  | spawn cap => simp only [step]; split <;> rfl
  | loadElf e => simp only [step]; split <;> rfl
  | bindMsix i => simp only [step]; split <;> rfl
  | acquire pid n => rfl

/-- The DMA log as an unwinding datum. -/
def dmaUnwinding : Unwinding (List (DmaMap.Req × DmaMap.Claim)) where
  view := fun s => s.dma
  touches := TouchesDma
  step_frame := step_preserves_dma

/-- Non-interference for the admitted-capsule set, an instance of the framework:
    a trace with no spawn leaves the admitted set of another observer unchanged. -/
theorem admitted_noninterference (s s' : State) (tr tr' : List Syscall)
    (hagree : s.admitted = s'.admitted)
    (h : ∀ sc ∈ tr, ¬ TouchesAdmitted sc) (h' : ∀ sc ∈ tr', ¬ TouchesAdmitted sc) :
    (run s tr).admitted = (run s' tr').admitted :=
  admittedUnwinding.noninterference s s' tr tr' hagree h h'

/-- Non-interference for the DMA log, an instance of the framework: a trace with
    no DMA mapping leaves the DMA log of another observer unchanged. -/
theorem dma_noninterference (s s' : State) (tr tr' : List Syscall)
    (hagree : s.dma = s'.dma)
    (h : ∀ sc ∈ tr, ¬ TouchesDma sc) (h' : ∀ sc ∈ tr', ¬ TouchesDma sc) :
    (run s tr).dma = (run s' tr').dma :=
  dmaUnwinding.noninterference s s' tr tr' hagree h h'

/-! ### The remaining state components

For completeness the last four components of the machine state, the user-copy
log, the anti-rollback floor, the ELF-load log and the interrupt-bind log, are
each an unwinding instance as well. Every observable component of the state is
therefore non-interfering: nothing an observer is not part of can be seen in, or
leak into, any component it observes. -/

/-- Only a `userCopy` transition changes the accepted-copy log. -/
def TouchesCopies : Syscall → Prop
  | .userCopy _ _ => True
  | _ => False

theorem step_preserves_copies (s : State) (sc : Syscall)
    (h : ¬ TouchesCopies sc) : (step s sc).copies = s.copies := by
  cases sc with
  | userCopy addr len => exact absurd trivial h
  | attenuate pid mask => rfl
  | transfer src dst b => simp only [step]; split <;> rfl
  | revoke pid b => rfl
  | mapPage page p => simp only [step]; split <;> rfl
  | boot v => rfl
  | spawn cap => simp only [step]; split <;> rfl
  | dmaMap r c => simp only [step]; split <;> rfl
  | loadElf e => simp only [step]; split <;> rfl
  | bindMsix i => simp only [step]; split <;> rfl
  | acquire pid n => rfl

def copiesUnwinding : Unwinding (List (Nat × Nat)) where
  view := fun s => s.copies
  touches := TouchesCopies
  step_frame := step_preserves_copies

/-- Only a `boot` transition changes the anti-rollback floor. -/
def TouchesFloor : Syscall → Prop
  | .boot _ => True
  | _ => False

theorem step_preserves_floor (s : State) (sc : Syscall)
    (h : ¬ TouchesFloor sc) : (step s sc).floor = s.floor := by
  cases sc with
  | boot v => exact absurd trivial h
  | attenuate pid mask => rfl
  | transfer src dst b => simp only [step]; split <;> rfl
  | revoke pid b => rfl
  | mapPage page p => simp only [step]; split <;> rfl
  | userCopy addr len => simp only [step]; split <;> rfl
  | spawn cap => simp only [step]; split <;> rfl
  | dmaMap r c => simp only [step]; split <;> rfl
  | loadElf e => simp only [step]; split <;> rfl
  | bindMsix i => simp only [step]; split <;> rfl
  | acquire pid n => rfl

def floorUnwinding : Unwinding Nat where
  view := fun s => s.floor
  touches := TouchesFloor
  step_frame := step_preserves_floor

/-- Only a `loadElf` transition changes the ELF-load log. -/
def TouchesElf : Syscall → Prop
  | .loadElf _ => True
  | _ => False

theorem step_preserves_elf (s : State) (sc : Syscall)
    (h : ¬ TouchesElf sc) : (step s sc).elf = s.elf := by
  cases sc with
  | loadElf e => exact absurd trivial h
  | attenuate pid mask => rfl
  | transfer src dst b => simp only [step]; split <;> rfl
  | revoke pid b => rfl
  | mapPage page p => simp only [step]; split <;> rfl
  | userCopy addr len => simp only [step]; split <;> rfl
  | boot v => rfl
  | spawn cap => simp only [step]; split <;> rfl
  | dmaMap r c => simp only [step]; split <;> rfl
  | bindMsix i => simp only [step]; split <;> rfl
  | acquire pid n => rfl

def elfUnwinding : Unwinding (List ElfSpec) where
  view := fun s => s.elf
  touches := TouchesElf
  step_frame := step_preserves_elf

/-- Only a `bindMsix` transition changes the interrupt-bind log. -/
def TouchesIrq : Syscall → Prop
  | .bindMsix _ => True
  | _ => False

theorem step_preserves_irq (s : State) (sc : Syscall)
    (h : ¬ TouchesIrq sc) : (step s sc).irq = s.irq := by
  cases sc with
  | bindMsix i => exact absurd trivial h
  | acquire pid n => rfl
  | attenuate pid mask => rfl
  | transfer src dst b => simp only [step]; split <;> rfl
  | revoke pid b => rfl
  | mapPage page p => simp only [step]; split <;> rfl
  | userCopy addr len => simp only [step]; split <;> rfl
  | boot v => rfl
  | spawn cap => simp only [step]; split <;> rfl
  | dmaMap r c => simp only [step]; split <;> rfl
  | loadElf e => simp only [step]; split <;> rfl

def irqUnwinding : Unwinding (List IrqBind.Input) where
  view := fun s => s.irq
  touches := TouchesIrq
  step_frame := step_preserves_irq

/-- Non-interference for the accepted-copy log. -/
theorem copies_noninterference (s s' : State) (tr tr' : List Syscall)
    (hagree : s.copies = s'.copies)
    (h : ∀ sc ∈ tr, ¬ TouchesCopies sc) (h' : ∀ sc ∈ tr', ¬ TouchesCopies sc) :
    (run s tr).copies = (run s' tr').copies :=
  copiesUnwinding.noninterference s s' tr tr' hagree h h'

/-- Non-interference for the anti-rollback floor. -/
theorem floor_noninterference (s s' : State) (tr tr' : List Syscall)
    (hagree : s.floor = s'.floor)
    (h : ∀ sc ∈ tr, ¬ TouchesFloor sc) (h' : ∀ sc ∈ tr', ¬ TouchesFloor sc) :
    (run s tr).floor = (run s' tr').floor :=
  floorUnwinding.noninterference s s' tr tr' hagree h h'

/-- Non-interference for the ELF-load log. -/
theorem elf_noninterference (s s' : State) (tr tr' : List Syscall)
    (hagree : s.elf = s'.elf)
    (h : ∀ sc ∈ tr, ¬ TouchesElf sc) (h' : ∀ sc ∈ tr', ¬ TouchesElf sc) :
    (run s tr).elf = (run s' tr').elf :=
  elfUnwinding.noninterference s s' tr tr' hagree h h'

/-- Non-interference for the interrupt-bind log. -/
theorem irq_noninterference (s s' : State) (tr tr' : List Syscall)
    (hagree : s.irq = s'.irq)
    (h : ∀ sc ∈ tr, ¬ TouchesIrq sc) (h' : ∀ sc ∈ tr', ¬ TouchesIrq sc) :
    (run s tr).irq = (run s' tr').irq :=
  irqUnwinding.noninterference s s' tr tr' hagree h h'

/-! ### Per-domain observational equivalence (confidentiality)

The instances above isolate one component of the whole state at a time. The
statement a capability system ultimately wants is per domain: a capsule observes
its own authority and the mappings of the pages it owns, and nothing another
capsule does outside that footprint changes what it observes. With an ownership
relation `owns q pg` the combined observation is again an unwinding instance, so
the confidentiality result falls out of the same framework: two states that look
identical to a domain stay identical to it under any two traces that never touch
its footprint. -/

/-- A domain's observation: its authority, and the installed permission of every
    page it owns. Pages it does not own read as `none`, so they are invisible to
    it and cannot enter its observation. -/
def DomainView (owns : Nat → Nat → Bool) (q : Nat) (s : State) :
    (Nat → Bool) × (Nat → Option Isolation.Perm) :=
  (s.token q, fun pg => if owns q pg = true then s.mapped pg else none)

/-- A transition touches domain `q` if it names `q`'s authority or maps a page
    that `q` owns. Every other transition is outside `q`'s footprint. -/
def TouchesDomain (owns : Nat → Nat → Bool) (q : Nat) : Syscall → Prop
  | .attenuate pid _ => pid = q
  | .transfer _ dst _ => dst = q
  | .revoke pid _ => pid = q
  | .mapPage page _ => owns q page = true
  | _ => False

/-- Touching a domain's token is a special case of touching the domain: the
    authority-naming transitions are shared between the two predicates. -/
theorem touchesToken_imp_domain (owns : Nat → Nat → Bool) (q : Nat) (sc : Syscall)
    (h : TouchesToken q sc) : TouchesDomain owns q sc := by
  cases sc with
  | attenuate pid mask => exact h
  | transfer src dst b => exact h
  | revoke pid b => exact h
  | mapPage page p => exact h.elim
  | userCopy addr len => exact h.elim
  | boot v => exact h.elim
  | spawn cap => exact h.elim
  | dmaMap r c => exact h.elim
  | loadElf e => exact h.elim
  | bindMsix i => exact h.elim
  | acquire pid n => exact h.elim

/-- The combined frame lemma: a transition outside domain `q`'s footprint leaves
    both its authority and its owned-page mappings, and so its whole observation,
    exactly as they were. -/
theorem step_preserves_domain_view (owns : Nat → Nat → Bool) (q : Nat) (s : State)
    (sc : Syscall) (h : ¬ TouchesDomain owns q sc) :
    DomainView owns q (step s sc) = DomainView owns q s := by
  have htok : (step s sc).token q = s.token q :=
    step_preserves_token q s sc (fun ht => h (touchesToken_imp_domain owns q sc ht))
  have hmap : ∀ pg, owns q pg = true → (step s sc).mapped pg = s.mapped pg := by
    intro pg hown
    apply step_preserves_mapping pg s sc
    intro htp
    cases sc with
    | mapPage page p =>
      have hpage : owns q page = true := by
        have : page = pg := htp
        rw [this]; exact hown
      exact h hpage
    | attenuate pid mask => exact htp.elim
    | transfer src dst b => exact htp.elim
    | revoke pid b => exact htp.elim
    | userCopy addr len => exact htp.elim
    | boot v => exact htp.elim
    | spawn cap => exact htp.elim
    | dmaMap r c => exact htp.elim
    | loadElf e => exact htp.elim
    | bindMsix i => exact htp.elim
    | acquire pid n => exact htp.elim
  have hmapfn :
      (fun pg => if owns q pg = true then (step s sc).mapped pg else (none : Option Isolation.Perm))
        = (fun pg => if owns q pg = true then s.mapped pg else none) := by
    funext pg
    by_cases hown : owns q pg = true
    · rw [if_pos hown, if_pos hown, hmap pg hown]
    · rw [if_neg hown, if_neg hown]
  unfold DomainView
  rw [htok, hmapfn]

/-- The domain observation as an unwinding datum. -/
def domainUnwinding (owns : Nat → Nat → Bool) (q : Nat) :
    Unwinding ((Nat → Bool) × (Nat → Option Isolation.Perm)) where
  view := DomainView owns q
  touches := TouchesDomain owns q
  step_frame := step_preserves_domain_view owns q

/-- Confidentiality, as observational equivalence: two states that a domain `q`
    cannot tell apart stay indistinguishable to it after any two traces that
    never touch its footprint. Whatever the rest of the system does, and however
    the adversary interleaves it, `q`'s view of its own authority and memory is
    unchanged. This is the per-domain non-interference the whole development is
    aimed at, and it is an instance of the same unwinding framework. -/
theorem domain_noninterference (owns : Nat → Nat → Bool) (q : Nat) (s s' : State)
    (tr tr' : List Syscall)
    (hagree : DomainView owns q s = DomainView owns q s')
    (h : ∀ sc ∈ tr, ¬ TouchesDomain owns q sc)
    (h' : ∀ sc ∈ tr', ¬ TouchesDomain owns q sc) :
    DomainView owns q (run s tr) = DomainView owns q (run s' tr') :=
  (domainUnwinding owns q).noninterference s s' tr tr' hagree h h'

/-! ### Multi-domain spatial isolation

Two capsules with disjoint footprints, distinct identities and no page in common,
never touch each other: a transition inside one's footprint is always outside the
other's. So all of one capsule's activity leaves the other's observation exactly
as it was. This is the separation guarantee a capability microkernel exists to
provide, and it composes directly from the per-domain view. -/

/-- Two domains have disjoint footprints when they are distinct and own no page
    in common. -/
def DisjointFootprint (owns : Nat → Nat → Bool) (q1 q2 : Nat) : Prop :=
  q1 ≠ q2 ∧ ∀ pg, ¬ (owns q1 pg = true ∧ owns q2 pg = true)

/-- A transition inside one domain's footprint is outside a disjoint domain's:
    distinct capsules with disjoint memory never touch one another. -/
theorem touches_disjoint (owns : Nat → Nat → Bool) (q1 q2 : Nat)
    (hd : DisjointFootprint owns q1 q2) (sc : Syscall)
    (h : TouchesDomain owns q1 sc) : ¬ TouchesDomain owns q2 sc := by
  obtain ⟨hne, hpg⟩ := hd
  cases sc with
  | attenuate pid mask => intro h2; exact hne (h.symm.trans h2)
  | transfer src dst b => intro h2; exact hne (h.symm.trans h2)
  | revoke pid b => intro h2; exact hne (h.symm.trans h2)
  | mapPage page p => intro h2; exact hpg page ⟨h, h2⟩
  | userCopy addr len => exact h.elim
  | boot v => exact h.elim
  | spawn cap => exact h.elim
  | dmaMap r c => exact h.elim
  | loadElf e => exact h.elim
  | bindMsix i => exact h.elim
  | acquire pid n => exact h.elim

/-- Spatial isolation: a trace whose every transition lies in domain `q1`'s
    footprint leaves a disjoint domain `q2`'s observation exactly as it was. One
    capsule's activity is invisible to another it shares no memory with, whatever
    that activity is. -/
theorem domain_isolation (owns : Nat → Nat → Bool) (q1 q2 : Nat)
    (hd : DisjointFootprint owns q1 q2) (s : State) (tr : List Syscall)
    (h : ∀ sc ∈ tr, TouchesDomain owns q1 sc) :
    DomainView owns q2 (run s tr) = DomainView owns q2 s :=
  (domainUnwinding owns q2).locality s tr
    (fun sc hsc => touches_disjoint owns q1 q2 hd sc (h sc hsc))

/-- Two disjoint capsules do not interfere: starting from any two states `q2`
    cannot tell apart, after any two traces each confined to `q1`'s footprint,
    `q2` still cannot tell them apart. Confidentiality between capsules that share
    no memory, with the adversary driving `q1` however it likes. -/
theorem disjoint_domains_noninterfere (owns : Nat → Nat → Bool) (q1 q2 : Nat)
    (hd : DisjointFootprint owns q1 q2) (s s' : State) (tr tr' : List Syscall)
    (hagree : DomainView owns q2 s = DomainView owns q2 s')
    (h : ∀ sc ∈ tr, TouchesDomain owns q1 sc)
    (h' : ∀ sc ∈ tr', TouchesDomain owns q1 sc) :
    DomainView owns q2 (run s tr) = DomainView owns q2 (run s' tr') := by
  rw [domain_isolation owns q1 q2 hd s tr h,
      domain_isolation owns q1 q2 hd s' tr' h', hagree]

end Nonos.NonInterference
