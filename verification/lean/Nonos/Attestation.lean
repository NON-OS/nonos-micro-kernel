/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification-level proof of IPC sender attestation: the kernel stamps a
message's sender from the actual caller, so a capsule cannot impersonate
another. The real kernel code that stamps the sender is proven to match by the
runnable + fuzz harnesses in `userland/fs_proofs` (the caller-attestation /
`split_caller` proofs: no message can claim a sender it did not originate from).
Lean states the property; the code proofs discharge it on the implementation.
-/

namespace Nonos.Attestation

/-- An IPC message: the sender the kernel stamped, and its payload. -/
structure Message where
  sender : Nat
  payload : Nat

/-- The kernel builds a message by stamping the true caller as the sender and
    ignoring any sender the caller tried to claim. -/
def send (caller : Nat) (payload : Nat) (_claimed : Nat) : Message :=
  { sender := caller, payload := payload }

/-- A delivered message's sender is exactly the caller that sent it. -/
theorem sender_is_caller (caller payload claimed : Nat) :
    (send caller payload claimed).sender = caller := rfl

/-- The claimed sender has no effect: two sends by the same caller with different
    claimed senders carry the same stamped sender. There is no channel for a
    caller to set the sender field. -/
theorem sender_independent_of_claim (caller payload c1 c2 : Nat) :
    (send caller payload c1).sender = (send caller payload c2).sender := rfl

/-- No impersonation: a message sent by `caller` can never carry a different
    principal as its sender. -/
theorem no_impersonation (caller payload claimed other : Nat)
    (h : other ≠ caller) : (send caller payload claimed).sender ≠ other := by
  intro hc
  apply h
  rw [sender_is_caller] at hc
  exact hc.symm

/-- The stamp is exact: the delivered sender equals a principal if and only if
    that principal is the true caller. -/
theorem sender_iff_caller (caller payload claimed p : Nat) :
    (send caller payload claimed).sender = p ↔ p = caller := by
  rw [sender_is_caller]
  exact eq_comm

/-- The payload is delivered untouched: attestation stamps the sender without
    disturbing the message body. -/
theorem payload_preserved (caller payload claimed : Nat) :
    (send caller payload claimed).payload = payload := rfl

end Nonos.Attestation
