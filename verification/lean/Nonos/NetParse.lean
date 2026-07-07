/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification of the network parser safety invariants. Two properties carry
the security weight. First, slice confinement: a parser that accepts a frame
hands back a payload lying inside the received bytes, never a window past
them. `userland/net_proofs` discharges this on the real capsule parsers:
`tcp_parse_never_panics_and_payload_stays_in_bounds` and the Ethernet, IPv4,
and UDP bounds harnesses in `userland/fs_proofs` (`proof_eth_parse_total`,
`proof_ipv4_parse_total`, `proof_udp_parse_total`, Kani). Second, termination
of the DNS name walk: the classic attack is a compression pointer that loops,
hanging the resolver. The real `skip` in the DNS capsule bounds the walk by a
step budget and ends a name the moment a pointer byte appears; discharged by
`compression_pointers_terminate_and_do_not_loop` and
`first_address_never_panics_over_adversarial_messages` over every two-byte
pointer value, including self-referential ones.

The model mirrors the real mechanism, not just the outcome: `walk` spends one
unit of budget per label and returns on a terminator, a pointer, an
out-of-bounds read, or an exhausted budget. Termination is structural in the
budget, so it holds for every message, hostile or not.
-/

namespace Nonos.NetParse

/-- Slice confinement: a parser accepts only offsets and lengths whose window
    fits the `n` received bytes. -/
def payloadAccepted (n off len : Nat) : Bool := off + len ≤ n

/-- An accepted payload window lies inside the received frame. -/
theorem accepted_payload_in_bounds (n off len : Nat)
    (h : payloadAccepted n off len = true) : off + len ≤ n := by
  simpa [payloadAccepted] using h

/-- A window past the frame is rejected. -/
theorem escaping_payload_rejected (n off len : Nat)
    (h : n < off + len) : payloadAccepted n off len = false := by
  unfold payloadAccepted
  have : ¬ off + len ≤ n := by omega
  simp [this]

/-- One step of the DNS name walk, as the real `skip` sees it: the byte at
    the cursor is a terminator, a compression pointer, or a label of some
    length. -/
inductive NameByte where
  | terminator
  | pointer
  | label (len : Nat)

/-- The walk over a message of classified bytes. One label costs one unit of
    budget; a terminator or pointer ends the name; running out of message or
    budget is a detected error, never a hang. Recursion is structural in
    `budget`, so the walk terminates on every input by construction. -/
def walk (msg : Nat → NameByte) (n : Nat) : Nat → Nat → Option Nat
  | _, 0 => none
  | pos, budget + 1 =>
    if pos < n then
      match msg pos with
      | .terminator => some (pos + 1)
      | .pointer => some (pos + 2)
      | .label len => walk msg n (pos + 1 + len) budget
    else
      none

/-- A pointer byte ends the walk at once: the very construction that makes a
    pointer loop impossible. The walk never follows the pointer, so there is
    nothing to loop through. -/
theorem a_pointer_ends_the_walk (msg : Nat → NameByte) (n pos budget : Nat)
    (hpos : pos < n) (hb : msg pos = .pointer) :
    walk msg n pos (budget + 1) = some (pos + 2) := by
  simp [walk, hpos, hb]

/-- The walk never reads outside the message: a cursor at or past the end is
    an error, not an access. -/
theorem an_out_of_bounds_cursor_is_an_error (msg : Nat → NameByte)
    (n pos budget : Nat) (hpos : n ≤ pos) :
    walk msg n pos (budget + 1) = none := by
  have : ¬ pos < n := by omega
  simp [walk, this]

/-- The walk visits at most `budget` labels: every result it can return is
    reached within the budget, because each recursive step consumes one unit.
    Formally, whatever the message contents, the walk with budget zero is
    already decided, so no run outlives its budget. Together with
    `a_pointer_ends_the_walk` this is the loop-freedom of the real `skip`:
    labels are budgeted and pointers do not recurse. -/
theorem an_exhausted_budget_stops_the_walk (msg : Nat → NameByte) (n pos : Nat) :
    walk msg n pos 0 = none := by
  simp [walk]

end Nonos.NetParse
