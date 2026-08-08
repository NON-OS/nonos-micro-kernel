/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Demand-fault admission. The page-fault entry
(src/interrupts/handlers/exceptions/page_fault.rs) refuses any address on a
registered guard page before the paging manager runs. The fault dispatcher
(src/memory/paging/manager/faults/handler.rs) routes a write on a present page
to copy-on-write and only a not-present fault to the demand path; a present
non-write fault is unhandled and the process is killed. The demand path
(faults/demand.rs) backs a page only when the address is in the user half
(`in_user_space`, at or below USER_TOP), is not in the null page (below
PAGE_SIZE_4K), and demand_cap::charge admits the process: pid 0 always, any
other pid only while its counter is under MAX_DEMAND_PAGES, and the counter
never decreases. The tracking table holds MAX_TRACKED slots; a process that
finds none is charged against a shared overflow budget, which is the branch the
model previously left out. A served page is mapped READ | WRITE | USER, never
EXECUTE. The theorems show: a guarded or null-page or kernel-half address is
never mapped, a present-page fault is never demand-served, an over-budget
process is refused and stays refused forever, a saturated table refuses once
its overflow budget is spent and admits a bounded number of faults in total,
pid 0 is never charged, and serving never grants execute so it cannot create a
writable-and-executable mapping.
-/

namespace Nonos.DemandPaging

/-- PAGE_SIZE_4K: the null page is [0, 4096) and is never demand-backed. -/
def pageSize : Nat := 4096

/-- USER_TOP: `in_user_space va` is `va ≤ USER_TOP`. -/
def userTop : Nat := 0x00007FFFFFFFFFFF

/-- MAX_DEMAND_PAGES: the per-process demand budget (demand_cap.rs). -/
def maxDemandPages : Nat := 4096

/-- MAX_TRACKED: slots in the demand-cap tracking table (demand_cap.rs). A
    process that finds no slot is charged against the shared overflow budget
    rather than admitted unmetered. -/
def maxTracked : Nat := 128

/-- A page fault: the faulting address and the PF_PRESENT / PF_WRITE bits of
    the hardware error code. -/
structure Fault where
  addr : Nat
  present : Bool
  write : Bool

/-- Where handler.rs sends a fault: write on a present page to copy-on-write,
    a not-present fault to the demand path, anything else is unhandled. -/
inductive Route
  | cow
  | demand
  | unhandled

/-- The dispatch in handler.rs, in its exact order. -/
def route (f : Fault) : Route :=
  if f.write && f.present then .cow
  else if !f.present then .demand
  else .unhandled

/-- What demand_cap::charge sees when a fault arrives: the pages already
    charged to this process if it holds a slot, how many live processes hold
    slots otherwise, and the shared overflow budget already spent. -/
structure Charge where
  tracked : Option Nat
  livePids : Nat
  overflow : Nat

/-- demand_cap::charge admits a fault when the process is the kernel (pid 0),
    when it holds a slot and is under budget, when a slot is free to claim
    (fewer than MAX_TRACKED live processes hold one, dead slots being reclaimed
    first), or, with the table saturated, while the shared overflow budget is
    unspent. The last branch is the one the code used to admit unconditionally:
    modelling it is what makes this predicate say something the code can fail. -/
def allows (pid : Nat) (c : Charge) : Prop :=
  pid = 0 ∨
    match c.tracked with
    | some pages => pages < maxDemandPages
    | none => c.livePids < maxTracked ∨ c.overflow < maxDemandPages

/-- The overflow budget after one admission through the saturated path. -/
def nextOverflow (ov : Nat) : Nat := if ov < maxDemandPages then ov + 1 else ov

/-- Faults admitted through the saturated path over `n` charges, starting from
    `ov` already spent. Each admission spends one; once the budget is spent
    every later fault is refused. -/
def admittedSat (ov : Nat) : Nat → Nat
  | 0 => 0
  | n + 1 => if ov < maxDemandPages then 1 + admittedSat (ov + 1) n else 0

/-- The counter after one charge: pid 0 is never charged; a tracked counter is
    bumped while at or under the budget (including the one-shot log bump at
    exactly the budget) and saturates above it. It never decreases. -/
def next (pid pages : Nat) : Nat :=
  if pid = 0 then pages else if pages ≤ maxDemandPages then pages + 1 else pages

/-- The counter after `n` demand faults. -/
def nextN (pid pages : Nat) : Nat → Nat
  | 0 => pages
  | n + 1 => nextN pid (next pid pages) n

/-- demand.rs serves a fault only when no guard page covers the address, the
    page is not present, the address is above the null page and inside the
    user half, and the budget admits the process. -/
def serves (guard : Nat → Prop) (f : Fault) (pid : Nat) (c : Charge) : Prop :=
  ¬ guard f.addr ∧ f.present = false ∧ pageSize ≤ f.addr ∧ f.addr ≤ userTop ∧
    allows pid c

/-- Permission bits of a mapping. -/
structure Perm where
  read : Bool
  write : Bool
  exec : Bool
  user : Bool

/-- The fixed permissions of a demand-backed page: READ | WRITE | USER. -/
def demandPerms : Perm := ⟨true, true, false, true⟩

/-- A W^X violation as the paging layer defines it. -/
def wx (p : Perm) : Prop := p.write = true ∧ p.exec = true

/-! ### Dispatch -/

/-- A not-present fault is routed to the demand path. -/
theorem route_not_present (f : Fault) (h : f.present = false) :
    route f = .demand := by simp [route, h]

/-- A write to a present page goes to copy-on-write, never to demand. -/
theorem route_write_present (f : Fault) (hw : f.write = true)
    (hp : f.present = true) : route f = .cow := by simp [route, hw, hp]

/-- A present-page fault that is not a write is unhandled: the process is
    killed and the mapping is never widened. -/
theorem route_present_read (f : Fault) (hw : f.write = false)
    (hp : f.present = true) : route f = .unhandled := by simp [route, hw, hp]

/-! ### Admission -/

/-- A guard-page address is never demand-mapped: page_fault.rs refuses it
    before the paging manager runs. -/
theorem guard_never_mapped (guard : Nat → Prop) (f : Fault) (pid : Nat)
    (c : Charge) (h : guard f.addr) : ¬ serves guard f pid c :=
  fun hs => hs.1 h

/-- The null page is never demand-backed: a near-null dereference traps
    instead of being silently satisfied. -/
theorem null_page_never_mapped (guard : Nat → Prop) (f : Fault)
    (pid : Nat) (c : Charge) (h : f.addr < pageSize) :
    ¬ serves guard f pid c := by
  intro hs; have := hs.2.2.1; omega

/-- No kernel-half address is ever demand-backed: backing one would hand a
    capsule kernel-range memory. -/
theorem kernel_half_never_mapped (guard : Nat → Prop) (f : Fault)
    (pid : Nat) (c : Charge) (h : userTop < f.addr) :
    ¬ serves guard f pid c := by
  intro hs; have := hs.2.2.2.1; omega

/-- An address outside the only demand-backable window [pageSize, userTop] is
    never mapped. -/
theorem outside_window_never_mapped (guard : Nat → Prop) (f : Fault)
    (pid : Nat) (c : Charge) (h : f.addr < pageSize ∨ userTop < f.addr) :
    ¬ serves guard f pid c := by
  intro hs; have h1 := hs.2.2.1; have h2 := hs.2.2.2.1; omega

/-- Only a not-present fault is demand-served. -/
theorem present_fault_not_served (guard : Nat → Prop) (f : Fault)
    (pid : Nat) (c : Charge) (h : f.present = true) :
    ¬ serves guard f pid c := by
  intro hs; have hp := hs.2.1; rw [h] at hp; exact Bool.noConfusion hp

/-! ### Budget -/

/-- pid 0 (a fault with no current process) is never charged. -/
theorem kernel_never_charged (pages : Nat) : next 0 pages = pages := by
  simp [next]

/-- One charge never decreases the counter. -/
theorem next_monotone (pid pages : Nat) : pages ≤ next pid pages := by
  simp only [next]
  split
  · omega
  · split <;> omega

/-- Any number of charges never decreases the counter. -/
theorem nextN_monotone (pid pages n : Nat) : pages ≤ nextN pid pages n := by
  induction n generalizing pages with
  | zero => simp [nextN]
  | succ k ih =>
    have h1 := next_monotone pid pages
    have h2 := ih (next pid pages)
    simp only [nextN]
    omega

/-- An over-budget process is refused: the fault path kills it instead of
    letting it exhaust physical memory. -/
theorem over_budget_refused (guard : Nat → Prop) (f : Fault) (pid pages : Nat)
    (c : Charge) (hc : c.tracked = some pages)
    (hpid : pid ≠ 0) (h : maxDemandPages ≤ pages) :
    ¬ serves guard f pid c := by
  intro hs
  rcases hs.2.2.2.2 with h0 | hb
  · exact hpid h0
  · simp only [hc] at hb; omega

/-- Refusal is permanent: the counter never decreases, so once a process is
    over budget no later fault is ever served again. -/
theorem refused_forever (guard : Nat → Prop) (f : Fault) (pid pages n : Nat)
    (c : Charge) (hc : c.tracked = some (nextN pid pages n))
    (hpid : pid ≠ 0) (h : maxDemandPages ≤ pages) :
    ¬ serves guard f pid c := by
  have hm := nextN_monotone pid pages n
  exact over_budget_refused guard f pid _ c hc hpid (by omega)

/-! ### The saturated table -/

/-- With every slot held by a live process and the overflow budget spent, an
    untracked process is refused. The code admitted this case unconditionally
    until the overflow budget was added, which made 128 processes holding slots
    enough to leave the next process demand-backing memory with no cap at all;
    stated this way the theorem is false of that code and true of the fix. -/
theorem saturated_refused (guard : Nat → Prop) (f : Fault) (pid : Nat)
    (c : Charge) (hpid : pid ≠ 0) (ht : c.tracked = none)
    (hlive : maxTracked ≤ c.livePids) (hov : maxDemandPages ≤ c.overflow) :
    ¬ serves guard f pid c := by
  intro hs
  rcases hs.2.2.2.2 with h0 | hb
  · exact hpid h0
  · simp only [ht] at hb
    rcases hb with hl | ho
    · omega
    · omega

/-- The overflow budget never decreases while the table stays saturated, so a
    refusal there holds until a slot frees. -/
theorem nextOverflow_monotone (ov : Nat) : ov ≤ nextOverflow ov := by
  simp only [nextOverflow]; split <;> omega

/-- Once the overflow budget is spent, no further fault is admitted through the
    saturated path. -/
theorem overflow_spent_refuses (ov n : Nat) (h : maxDemandPages ≤ ov) :
    admittedSat ov n = 0 := by
  cases n with
  | zero => simp [admittedSat]
  | succ k => simp only [admittedSat]; rw [if_neg (by omega)]

/-- The saturated path admits at most the pages the budget still holds. -/
theorem admittedSat_le (ov n : Nat) : admittedSat ov n ≤ maxDemandPages - ov := by
  induction n generalizing ov with
  | zero => simp [admittedSat]
  | succ k ih =>
    simp only [admittedSat]
    split
    · have := ih (ov + 1); omega
    · omega

/-- However many faults arrive with the table saturated, the path around the
    per-process counter serves at most one process's worth of pages in total.
    This is the bound the unconditional admission did not have. -/
theorem saturated_admission_bounded (n : Nat) :
    admittedSat 0 n ≤ maxDemandPages := by
  have h := admittedSat_le 0 n
  omega

/-! ### Permissions -/

/-- A served page is never executable: demand backing cannot create code. -/
theorem demand_never_exec : demandPerms.exec = false := rfl

/-- A demand-backed page is never a W^X violation. -/
theorem demand_not_wx : ¬ wx demandPerms := by
  simp [wx, demandPerms]

end Nonos.DemandPaging
