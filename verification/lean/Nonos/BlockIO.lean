/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification of block I/O request bounds. A storage request names a starting
sector and a sector count; the security property is that an accepted request
addresses only sectors that exist, so no transfer the driver programs can
reach past the disk. `userland/driver_proofs` discharges this on the real AHCI
request parser: `rw_parse_is_total_and_bounded` (runnable and Kani, all
requests), `rw_parse_never_panics_and_requests_stay_within_the_disk`, and
`short_bodies_are_rejected_not_panicked`. The code proofs also show `lba +
count` never wraps in the machine integers, which is what entitles this model
to plain natural-number arithmetic.
-/

namespace Nonos.BlockIO

/-- The driver's acceptance gate: a nonzero count, at most the per-request
    maximum, ending at or before the last sector of a `capacity`-sector disk. -/
def accepts (capacity maxCount lba count : Nat) : Bool :=
  1 ≤ count && count ≤ maxCount && lba + count ≤ capacity

/-- An accepted request stays on the disk: its last sector exists. -/
theorem accepted_request_stays_on_disk (capacity maxCount lba count : Nat)
    (h : accepts capacity maxCount lba count = true) :
    lba + count ≤ capacity := by
  unfold accepts at h
  simp [Bool.and_eq_true] at h
  exact h.2

/-- A request reaching past the disk is rejected, whatever its count. -/
theorem an_escaping_request_is_rejected (capacity maxCount lba count : Nat)
    (h : capacity < lba + count) :
    accepts capacity maxCount lba count = false := by
  unfold accepts
  have : ¬ lba + count ≤ capacity := by omega
  simp [this]

/-- A zero count and an oversized count are both rejected: the accepted
    counts are exactly `1 ..= maxCount`, so a transfer is never empty and
    never larger than the driver's own bound. -/
theorem a_zero_or_oversized_count_is_rejected (capacity maxCount lba count : Nat)
    (h : count = 0 ∨ maxCount < count) :
    accepts capacity maxCount lba count = false := by
  unfold accepts
  cases h with
  | inl h0 => simp [h0]
  | inr hbig =>
    have : ¬ count ≤ maxCount := by omega
    simp [this]

end Nonos.BlockIO
