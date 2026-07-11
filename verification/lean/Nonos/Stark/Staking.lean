/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The staking rewards accounting, and its no-inflation invariant. NOX staking pays out
of real protocol fees only, never from emissions, so the claim it makes is that
total rewards distributed can never exceed total fees funded. This module models the
reward pool as the pair (funded, distributed) and proves the invariant that
distributed stays within funded through every funding and every claim, and therefore
across any sequence of them. A claim beyond what is funded is rejected. The yield is
a direct claim on fee revenue, and nothing is minted.
-/

namespace Nonos.Stark.Staking

/-- The reward pool: total fees funded in, total rewards claimed out. -/
structure Rewards where
  funded : Int
  distributed : Int

/-- No inflation: rewards paid never exceed fees funded, and both stay nonnegative. -/
def Sound (r : Rewards) : Prop :=
  0 ≤ r.distributed ∧ r.distributed ≤ r.funded

/-- Fund the pool with fee revenue (a nonnegative amount). -/
def fund (r : Rewards) (amt : Int) : Rewards :=
  { r with funded := r.funded + amt }

/-- Claim rewards, gated by what remains: a claim beyond the funded balance, or a
    negative claim, is rejected. -/
def claim (r : Rewards) (amt : Int) : Option Rewards :=
  if 0 ≤ amt ∧ r.distributed + amt ≤ r.funded then
    some { r with distributed := r.distributed + amt }
  else none

/-- Funding with nonnegative revenue preserves soundness. -/
theorem fund_preserves_sound (r : Rewards) (amt : Int) (hamt : 0 ≤ amt)
    (h : Sound r) : Sound (fund r amt) := by
  obtain ⟨h1, h2⟩ := h
  refine ⟨h1, ?_⟩
  show r.distributed ≤ r.funded + amt
  omega

/-- A successful claim preserves soundness: it never pays more than is funded. -/
theorem claim_preserves_sound (r r' : Rewards) (amt : Int)
    (h : Sound r) (hc : claim r amt = some r') : Sound r' := by
  unfold claim at hc
  by_cases hcond : 0 ≤ amt ∧ r.distributed + amt ≤ r.funded
  · rw [if_pos hcond] at hc
    simp only [Option.some.injEq] at hc
    unfold Sound at *
    rw [← hc]
    show 0 ≤ r.distributed + amt ∧ r.distributed + amt ≤ r.funded
    omega
  · rw [if_neg hcond] at hc; exact absurd hc (by simp)

/-- An overdraw is rejected: you cannot claim more than the pool was funded. -/
theorem an_overdraw_is_rejected (r : Rewards) (amt : Int)
    (h : r.funded < r.distributed + amt) : claim r amt = none := by
  unfold claim
  rw [if_neg (by intro hcond; omega)]

/-- A pool action: fund with fee revenue, or claim rewards. -/
inductive Action
  | fund (amt : Int)
  | claim (amt : Int)

/-- One step. A rejected claim leaves the pool unchanged, so the machine is total. -/
def step (r : Rewards) : Action → Rewards
  | .fund amt => if 0 ≤ amt then fund r amt else r
  | .claim amt => (claim r amt).getD r

/-- Every action preserves soundness. -/
theorem step_preserves_sound (r : Rewards) (a : Action) (h : Sound r) :
    Sound (step r a) := by
  cases a with
  | fund amt =>
    simp only [step]
    by_cases hamt : 0 ≤ amt
    · rw [if_pos hamt]; exact fund_preserves_sound r amt hamt h
    · rw [if_neg hamt]; exact h
  | claim amt =>
    simp only [step]
    cases hc : claim r amt with
    | none => simpa only [Option.getD] using h
    | some r' =>
      simp only [Option.getD]
      exact claim_preserves_sound r r' amt h hc

/-- No inflation over all traces: from a sound start, any sequence of fundings and
    claims leaves total rewards within total fees. Nothing is ever minted. -/
theorem rewards_never_exceed_fees (r0 : Rewards) (actions : List Action)
    (h : Sound r0) : Sound (actions.foldl step r0) := by
  induction actions generalizing r0 with
  | nil => simpa using h
  | cons a as ih =>
    simp only [List.foldl_cons]
    exact ih (step r0 a) (step_preserves_sound r0 a h)

end Nonos.Stark.Staking
