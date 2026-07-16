/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Memory-grant conservation. The page allocator hands physical pages to capsules
(grant) and takes them back (release). The theorems below show the total page
count is an invariant of every reachable state, no sequence of grants and
releases ever creates or destroys a page, and that the granted count never
exceeds capacity. This is the kernel analogue of the STARK pool-solvency
proofs, landed on the physical page ledger.
-/

namespace Nonos.MemGrant

/-- The page ledger: pages still free to hand out, and pages currently granted
    to capsules. Their sum is the fixed physical capacity. -/
structure Ledger where
  free : Nat
  granted : Nat

/-- The capacity a ledger accounts for. -/
def capacity (l : Ledger) : Nat := l.free + l.granted

/-- An allocator request. -/
inductive Op
  | grant (n : Nat)
  | release (n : Nat)

/-- Service one request. A grant of `n` succeeds only if `n` pages are free; a
    release of `n` only if `n` are granted. A request that cannot be honored
    leaves the ledger unchanged rather than going negative. -/
def step (l : Ledger) : Op → Ledger
  | Op.grant n => if n ≤ l.free then ⟨l.free - n, l.granted + n⟩ else l
  | Op.release n => if n ≤ l.granted then ⟨l.free + n, l.granted - n⟩ else l

/-- Service a whole sequence of requests. -/
def run (l : Ledger) : List Op → Ledger
  | [] => l
  | op :: rest => run (step l op) rest

/-- One request conserves capacity. -/
theorem step_conserves (l : Ledger) (op : Op) : capacity (step l op) = capacity l := by
  cases op with
  | grant n =>
    by_cases h : n ≤ l.free
    · simp only [step, if_pos h, capacity]; omega
    · simp only [step, if_neg h]
  | release n =>
    by_cases h : n ≤ l.granted
    · simp only [step, if_pos h, capacity]; omega
    · simp only [step, if_neg h]

/-- CONSERVATION. No sequence of grants and releases ever creates or destroys a
    page: capacity is invariant across every reachable state. -/
theorem run_conserves (l : Ledger) (ops : List Op) : capacity (run l ops) = capacity l := by
  induction ops generalizing l with
  | nil => rfl
  | cons op rest ih =>
    unfold run
    rw [ih (step l op), step_conserves l op]

/-- Granted pages never exceed the capacity of the ledger. -/
theorem granted_le_capacity (l : Ledger) : l.granted ≤ capacity l := by
  unfold capacity; omega

/-- NO OVER-GRANT. After any request sequence, the pages granted out never
    exceed the physical capacity the allocator started with. -/
theorem run_granted_le_capacity (l : Ledger) (ops : List Op) :
    (run l ops).granted ≤ capacity l := by
  have hc := run_conserves l ops
  have hg := granted_le_capacity (run l ops)
  omega

/-- Free pages never exceed capacity either. -/
theorem run_free_le_capacity (l : Ledger) (ops : List Op) :
    (run l ops).free ≤ capacity l := by
  have hc := run_conserves l ops
  have hf : (run l ops).free ≤ capacity (run l ops) := by unfold capacity; omega
  omega

/-- A grant that fits reduces the free pool by exactly the amount granted. -/
theorem grant_moves_pages (l : Ledger) (n : Nat) (h : n ≤ l.free) :
    (step l (Op.grant n)).granted = l.granted + n := by
  simp only [step, if_pos h]

/-- A release that fits returns exactly the amount to the free pool. -/
theorem release_returns_pages (l : Ledger) (n : Nat) (h : n ≤ l.granted) :
    (step l (Op.release n)).free = l.free + n := by
  simp only [step, if_pos h]

/-- An over-large grant is refused: the ledger is left untouched, so a capsule
    can never be handed pages that are not free. -/
theorem oversized_grant_refused (l : Ledger) (n : Nat) (h : l.free < n) :
    step l (Op.grant n) = l := by
  simp only [step, if_neg (show ¬ n ≤ l.free by omega)]

/-- An over-large release is refused: pages cannot be returned that were never
    granted, so the granted count never underflows. -/
theorem oversized_release_refused (l : Ledger) (n : Nat) (h : l.granted < n) :
    step l (Op.release n) = l := by
  simp only [step, if_neg (show ¬ n ≤ l.granted by omega)]

end Nonos.MemGrant
