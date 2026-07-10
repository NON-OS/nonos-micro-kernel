/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The shielded pool as a state machine, and its solvency invariant. The pool holds a
balance of the asset and tracks the total value of live shielded notes; solvency is
the property that the two are equal, every shielded coin backed one to one by a coin
the contract holds. A deposit raises both, a withdrawal lowers both and is gated
through the nullifier guard so a note cannot be consumed twice. This module proves
solvency is preserved by every operation and therefore by every reachable pool
state, whatever sequence of deposits and withdrawals an adversary chooses. It folds
the double-spend guard (Nonos.Stark.NullifierSet) into the withdraw step, so the
capstone is the on-chain solvency invariant an auditor would demand, proven over all
traces rather than tested on a few.
-/

import Nonos.Stark.NullifierSet

namespace Nonos.Stark.Pool

open Nonos.Stark.NullifierSet

/-- The pool state: the asset balance the contract holds, the total value of live
    shielded notes, and the set of spent nullifiers. -/
structure Pool where
  balance : Int
  shielded : Int
  spent : List Int

/-- Solvency: the held balance exactly backs the shielded total. -/
def Solvent (p : Pool) : Prop := p.balance = p.shielded

/-- A deposit shields value: it raises the balance and the shielded total together. -/
def deposit (p : Pool) (v : Int) : Pool :=
  { p with balance := p.balance + v, shielded := p.shielded + v }

/-- A withdrawal, gated by the nullifier guard: it succeeds only on a fresh
    nullifier, and then lowers the balance and the shielded total together and
    records the nullifier. A stale nullifier is rejected. -/
def withdraw (p : Pool) (v nf : Int) : Option Pool :=
  match spend p.spent nf with
  | none => none
  | some s => some { balance := p.balance - v, shielded := p.shielded - v, spent := s }

/-- A deposit preserves solvency. -/
theorem deposit_preserves_solvency (p : Pool) (v : Int) (h : Solvent p) :
    Solvent (deposit p v) := by
  unfold Solvent deposit at *; simp only []; omega

/-- A successful withdrawal preserves solvency: it debits both sides equally, so the
    backing stays exact. -/
theorem withdraw_preserves_solvency (p p' : Pool) (v nf : Int)
    (h : Solvent p) (hw : withdraw p v nf = some p') : Solvent p' := by
  unfold withdraw at hw
  cases hs : spend p.spent nf with
  | none => rw [hs] at hw; exact absurd hw (by simp)
  | some s =>
    rw [hs] at hw
    simp only [Option.some.injEq] at hw
    unfold Solvent at *
    rw [← hw]
    show p.balance - v = p.shielded - v
    omega

/-- A pool operation: shield value in, or withdraw it out. -/
inductive Op
  | deposit (v : Int)
  | withdraw (v nf : Int)

/-- One step of the pool. A rejected withdrawal (stale nullifier) leaves the state
    unchanged, so the machine is total. -/
def step (p : Pool) : Op → Pool
  | .deposit v => deposit p v
  | .withdraw v nf => (withdraw p v nf).getD p

/-- Every operation preserves solvency, accepted or rejected. -/
theorem step_preserves_solvency (p : Pool) (o : Op) (h : Solvent p) :
    Solvent (step p o) := by
  cases o with
  | deposit v => exact deposit_preserves_solvency p v h
  | withdraw v nf =>
    simp only [step]
    cases hw : withdraw p v nf with
    | none => simpa only [Option.getD] using h
    | some p' =>
      simp only [Option.getD]
      exact withdraw_preserves_solvency p p' v nf h hw

/-- The capstone: every reachable pool state is solvent. Starting from a solvent
    pool, any sequence of deposits and withdrawals leaves the balance exactly
    backing the shielded total. The adversary picks every operation and never
    breaks the backing. -/
theorem every_reachable_pool_is_solvent (p0 : Pool) (ops : List Op)
    (h : Solvent p0) : Solvent (ops.foldl step p0) := by
  induction ops generalizing p0 with
  | nil => simpa using h
  | cons o os ih =>
    simp only [List.foldl_cons]
    exact ih (step p0 o) (step_preserves_solvency p0 o h)

end Nonos.Stark.Pool
