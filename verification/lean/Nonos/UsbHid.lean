/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification of the USB HID configuration-descriptor walk. The descriptor
bytes come entirely from the device, so the walk must terminate on hostile
input and can never yield more interface bindings than the driver's fixed
capacity. The classic defect is a zero-length descriptor record: a walk that
advances by the record's own length then never advances at all.
`userland/usb_proofs` discharges these on the real parser:
`descriptor_walk_terminates_and_binding_count_is_bounded` and
`hid_bindings_never_panics_over_adversarial_input`.

The model mirrors the real mechanism: each record leads with its length, a
zero length is rejected rather than stepped over, and every accepted record
advances the cursor by at least one byte, so the walk is decreasing in the
bytes that remain and Lean accepts it as terminating on every input.
-/

namespace Nonos.UsbHid

/-- The descriptor walk over the records the device supplied, each a length
    and whether it yields an interface binding. A zero length is a hard
    reject. The count never passes `cap`, mirroring the driver's fixed
    binding array. Each step drops the record's own bytes, so the recursion
    is on a strictly shorter list: termination for every device-supplied
    input is checked by Lean itself. -/
def walk (cap : Nat) : List (Nat × Bool) → Nat → Option Nat
  | [], count => some count
  | (len, binds) :: rest, count =>
    if len = 0 then
      none
    else
      walk cap (rest.drop (len - 1))
        (if binds && decide (count < cap) then count + 1 else count)
termination_by records _ => records.length
decreasing_by
  simp only [List.length_cons, List.length_drop]
  omega

/-- A zero-length record is rejected immediately: the walk reports an error
    instead of spinning on a cursor that no longer advances. -/
theorem a_zero_length_record_is_rejected (cap : Nat)
    (rest : List (Nat × Bool)) (binds : Bool) (count : Nat) :
    walk cap ((0, binds) :: rest) count = none := by
  simp [walk]

/-- One step never breaks the capacity invariant: a count within capacity
    stays within capacity, because the walk only increments below `cap`. -/
theorem a_step_preserves_the_cap (cap count : Nat) (binds : Bool)
    (h : count ≤ cap) :
    (if binds && decide (count < cap) then count + 1 else count) ≤ cap := by
  split
  · next hcond =>
    have : count < cap := by
      simp [Bool.and_eq_true] at hcond
      exact hcond.2
    omega
  · exact h

private theorem walk_bounded_aux (cap : Nat) :
    ∀ (n : Nat) (records : List (Nat × Bool)) (count out : Nat),
      records.length ≤ n → count ≤ cap → walk cap records count = some out →
      out ≤ cap := by
  intro n
  induction n with
  | zero =>
    intro records count out hlen hle h
    match records with
    | [] =>
      simp [walk] at h
      omega
    | _ :: _ =>
      simp [List.length_cons] at hlen
  | succ n ih =>
    intro records count out hlen hle h
    match records with
    | [] =>
      simp [walk] at h
      omega
    | (len, binds) :: rest =>
      by_cases h0 : len = 0
      · subst h0
        simp [walk] at h
      · simp only [walk, h0, if_false] at h
        refine ih (rest.drop (len - 1)) _ out ?_
          (a_step_preserves_the_cap cap count binds hle) h
        simp only [List.length_cons] at hlen
        simp only [List.length_drop]
        omega

/-- The binding count never exceeds the driver's fixed capacity: whatever
    records the device sends, a completed walk that started within capacity
    ends within capacity. The device chooses the records; it cannot choose
    the bound. Proven by induction on the bytes that remain, the same
    measure that makes the walk terminate. -/
theorem bindings_never_exceed_the_cap (cap : Nat) (records : List (Nat × Bool))
    (count out : Nat) (hle : count ≤ cap) (h : walk cap records count = some out) :
    out ≤ cap :=
  walk_bounded_aux cap records.length records count out (Nat.le_refl _) hle h

end Nonos.UsbHid
