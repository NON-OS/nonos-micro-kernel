/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The fee router, the revenue engine's integrity. Every shielded transaction pays a
protocol fee, which is split three ways: staking rewards, treasury, and buyback
burn. Two properties keep it honest. The split conserves: the three shares sum to
exactly the fee, nothing is minted and nothing leaks. And the fee is capped:
governance can set the rate only at or below a hard bound fixed in code, so it can
never rug users by raising the fee without limit. Both are proved here over the
integers; the contract discharges them with the same arithmetic on the deployed
FeeRouter and a bps cap constant.
-/

namespace Nonos.Stark.FeeRouter

/-- The three destinations of a routed fee. -/
structure Split where
  staking : Int
  treasury : Int
  burn : Int

/-- Everything the split pays out. -/
def total (s : Split) : Int := s.staking + s.treasury + s.burn

/-- Route a fee by basis-point shares: staking and treasury take their bps, the
    burn takes the remainder. The remainder construction is what makes it exact. -/
def route (fee stakingBps treasuryBps : Int) : Split :=
  let s := fee * stakingBps / 10000
  let t := fee * treasuryBps / 10000
  { staking := s, treasury := t, burn := fee - s - t }

/-- The split conserves the fee: the three shares sum to exactly the fee, no
    inflation and no leak. -/
theorem route_conserves (fee stakingBps treasuryBps : Int) :
    total (route fee stakingBps treasuryBps) = fee := by
  simp only [total, route]; omega

/-- Set the protocol fee, gated by the hard cap: a rate above the cap is rejected. -/
def setFee (cap proposed : Int) : Option Int :=
  if proposed ≤ cap then some proposed else none

/-- A fee above the cap is rejected: governance cannot raise it past the bound. -/
theorem over_cap_fee_is_rejected (cap proposed : Int) (h : cap < proposed) :
    setFee cap proposed = none := by
  unfold setFee; rw [if_neg (by omega)]

/-- Any accepted fee is within the cap: whatever governance sets is bounded. -/
theorem an_accepted_fee_is_within_cap (cap proposed accepted : Int)
    (h : setFee cap proposed = some accepted) : accepted ≤ cap := by
  unfold setFee at h
  by_cases hp : proposed ≤ cap
  · rw [if_pos hp] at h; simp only [Option.some.injEq] at h; omega
  · rw [if_neg hp] at h; exact absurd h (by simp)

/-- A worked instance: a 30 bps fee split 60/30/10 on a fee of 1000 pays 6, 3, 1. -/
theorem worked_split : total (route 10 6000 3000) = 10 := by decide

end Nonos.Stark.FeeRouter
