/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The hardware broker's DMA-map admission check
(`src/hardware/broker/dma/map/validate.rs::validate`). Before the broker hands a
device a DMA window it clears the request against the caller's claim: unknown
flags are refused, a length that is zero or not page aligned is refused, the
device must be claimed by exactly this caller, the claim epoch must match the
request so a stale claim cannot be replayed, and the page count must stay within
the device class's limit. The theorems fix those branches: an admitted mapping is
page aligned and non-empty, owned by the caller, on a fresh epoch, and bounded to
the class page limit, so no caller can program a device to DMA outside a window
it currently and legitimately owns.
-/

namespace Nonos.DmaMap

/-- The DMA page size, `PAGE_SIZE` in the broker. -/
def page : Nat := 4096

/-- The verdict, one constructor per `Result` arm of `validate`. -/
inductive Verdict where
  | ok
  | badFlags
  | badLength
  | notClaimed
  | staleEpoch
  | unknownDevice
  | badLengthForClass
  deriving DecidableEq

/-- The request fields the check reads. `flagsSupported` is the outcome of
    `flags & !FLAGS_KNOWN == 0`, kept as its boolean so the model stays in
    arithmetic the prover reasons about directly. -/
structure Req where
  flagsSupported : Bool
  length : Nat
  reqPid : Nat
  reqEpoch : Nat

/-- The claim and class state the check consults. -/
structure Claim where
  present : Bool
  pid : Nat
  epoch : Nat
  classPresent : Bool
  pageLimit : Nat

/-- `validate`, branch for branch. -/
def validate (r : Req) (c : Claim) : Verdict :=
  if r.flagsSupported = false then .badFlags
  else if r.length = 0 ∨ r.length % page ≠ 0 then .badLength
  else if c.present = false then .notClaimed
  else if c.pid ≠ r.reqPid then .notClaimed
  else if c.epoch ≠ r.reqEpoch then .staleEpoch
  else if c.classPresent = false then .unknownDevice
  else if r.length / page > c.pageLimit then .badLengthForClass
  else .ok

/-- Characterization of an admitted mapping: every guard passed. Every
    accepted-mapping property is read off this lemma. -/
theorem accepted (r : Req) (c : Claim) (h : validate r c = .ok) :
    r.flagsSupported = true ∧ r.length ≠ 0 ∧ r.length % page = 0 ∧
      c.present = true ∧ c.pid = r.reqPid ∧ c.epoch = r.reqEpoch ∧
      c.classPresent = true ∧ r.length / page ≤ c.pageLimit := by
  unfold validate at h
  by_cases hf : r.flagsSupported = false
  · rw [if_pos hf] at h; exact absurd h (by simp)
  · rw [if_neg hf] at h
    by_cases hl : r.length = 0 ∨ r.length % page ≠ 0
    · rw [if_pos hl] at h; exact absurd h (by simp)
    · rw [if_neg hl] at h
      by_cases hp : c.present = false
      · rw [if_pos hp] at h; exact absurd h (by simp)
      · rw [if_neg hp] at h
        by_cases ho : c.pid ≠ r.reqPid
        · rw [if_pos ho] at h; exact absurd h (by simp)
        · rw [if_neg ho] at h
          by_cases he : c.epoch ≠ r.reqEpoch
          · rw [if_pos he] at h; exact absurd h (by simp)
          · rw [if_neg he] at h
            by_cases hc : c.classPresent = false
            · rw [if_pos hc] at h; exact absurd h (by simp)
            · rw [if_neg hc] at h
              by_cases hlim : r.length / page > c.pageLimit
              · rw [if_pos hlim] at h; exact absurd h (by simp)
              · refine ⟨?_, ?_, ?_, ?_, ?_, ?_, ?_, ?_⟩
                · cases hb : r.flagsSupported <;> simp_all
                · exact fun hz => hl (Or.inl hz)
                · exact Decidable.of_not_not (fun hm => hl (Or.inr hm))
                · cases hb : c.present <;> simp_all
                · exact Decidable.of_not_not ho
                · exact Decidable.of_not_not he
                · cases hb : c.classPresent <;> simp_all
                · exact Nat.le_of_not_lt hlim

/-- An admitted mapping is a whole number of pages and non-empty: it never
    covers a partial page. -/
theorem accepted_page_aligned (r : Req) (c : Claim) (h : validate r c = .ok) :
    r.length ≠ 0 ∧ r.length % page = 0 :=
  ⟨(accepted r c h).2.1, (accepted r c h).2.2.1⟩

/-- An admitted mapping is owned by the caller: the claim's pid is the requester,
    so a caller can only map a device it holds. -/
theorem accepted_owned_by_caller (r : Req) (c : Claim) (h : validate r c = .ok) :
    c.pid = r.reqPid :=
  (accepted r c h).2.2.2.2.1

/-- An admitted mapping is on a fresh claim epoch: a stale or replayed claim can
    never admit a mapping. -/
theorem accepted_fresh_epoch (r : Req) (c : Claim) (h : validate r c = .ok) :
    c.epoch = r.reqEpoch :=
  (accepted r c h).2.2.2.2.2.1

/-- The bounded-DMA guarantee: an admitted mapping's page count is within the
    device class's limit, so no caller can program a device to DMA an unbounded
    region. -/
theorem accepted_within_class_limit (r : Req) (c : Claim) (h : validate r c = .ok) :
    r.length / page ≤ c.pageLimit :=
  (accepted r c h).2.2.2.2.2.2.2

/-- Unknown flag bits are refused outright. -/
theorem bad_flags_rejected (r : Req) (c : Claim) (hf : r.flagsSupported = false) :
    validate r c = .badFlags := by
  simp [validate, hf]

/-- A stale claim epoch is refused (given the earlier guards pass). -/
theorem stale_epoch_rejected (r : Req) (c : Claim)
    (hf : r.flagsSupported = true)
    (hl : ¬ (r.length = 0 ∨ r.length % page ≠ 0))
    (hp : c.present = true) (ho : c.pid = r.reqPid)
    (he : c.epoch ≠ r.reqEpoch) : validate r c = .staleEpoch := by
  unfold validate
  rw [if_neg (by simp [hf]), if_neg hl, if_neg (by simp [hp]), if_neg (by simp [ho]),
    if_pos he]

end Nonos.DmaMap
