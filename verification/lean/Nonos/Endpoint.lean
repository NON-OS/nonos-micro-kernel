/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

IPC endpoints. An endpoint is a FIFO queue of messages, each carrying its
sender's id. The theorems below show a delivered message was actually enqueued
(no message from thin air), the remaining messages keep their sender tags (no
tag rewritten in transit), and sending never drops a queued message, so a
receiver only ever sees messages that were really sent, tagged with the true
sender.
-/

namespace Nonos.Endpoint

/-- A message: the sender's id and its payload. -/
structure Msg where
  sender : Nat
  payload : Nat

/-- An endpoint queue, front first. -/
abbrev Queue := List Msg

/-- Enqueue a message at the back. -/
def send (q : Queue) (m : Msg) : Queue := q ++ [m]

/-- Take the front message, if any. -/
def recv : Queue → Option (Msg × Queue)
  | [] => none
  | m :: rest => some (m, rest)

/-- A sent message is in the queue afterward. -/
theorem send_mem (q : Queue) (m : Msg) : m ∈ send q m := by simp [send]

/-- Sending never drops a queued message. -/
theorem send_preserves (q : Queue) (m x : Msg) (hx : x ∈ q) : x ∈ send q m := by
  simp [send, hx]

/-- A delivered message was actually enqueued: nothing is received from thin
    air, and its sender tag is exactly the one it was sent with. -/
theorem recv_was_sent (q : Queue) (m : Msg) (rest : Queue)
    (h : recv q = some (m, rest)) : m ∈ q := by
  cases q with
  | nil => simp [recv] at h
  | cons a as =>
    simp only [recv, Option.some.injEq, Prod.mk.injEq] at h
    simp [h.1]

/-- The messages left after a delivery were all in the queue, tags intact: no
    sender is rewritten in transit. -/
theorem recv_rest_were_queued (q : Queue) (m : Msg) (rest : Queue)
    (h : recv q = some (m, rest)) (x : Msg) (hx : x ∈ rest) : x ∈ q := by
  cases q with
  | nil => simp [recv] at h
  | cons a as =>
    simp only [recv, Option.some.injEq, Prod.mk.injEq] at h
    rw [← h.2] at hx
    exact List.mem_cons_of_mem a hx

/-- Delivery empties nothing prematurely: the queue is empty exactly when there
    is nothing to receive. -/
theorem recv_none_iff (q : Queue) : recv q = none ↔ q = [] := by
  cases q with
  | nil => simp [recv]
  | cons a as => simp [recv]

/-- A delivered message keeps its sender: the tag the receiver sees is the tag
    the sender wrote. -/
theorem recv_sender (q : Queue) (m : Msg) (rest : Queue)
    (h : recv q = some (m, rest)) : ∃ w ∈ q, w.sender = m.sender := by
  exact ⟨m, recv_was_sent q m rest h, rfl⟩

end Nonos.Endpoint
