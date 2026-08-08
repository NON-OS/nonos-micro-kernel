/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

IPC reply correlation (`recv_reply_correlated` in
`src/syscall/microkernel/ipc/recv.rs`, the call token in
`src/syscall/microkernel/ipc/call/sys_ipc_call.rs`). A reply inbox is
kernel-owned on a predictable port, so any capsule could `mk_ipc_send` a forged
frame into another capsule's reply inbox. The fix tags each call with a nonzero
token and delivers only the message whose correlation equals that token,
dropping the rest. The theorems below are about that filter: a message with
correlation 0 is never delivered to a valid call, only a message carrying the
exact token is delivered, and an inbox full of forgeries yields nothing.

What the filter is worth depends on who can produce a matching correlation, and
that is not settled here. An earlier version of this comment said
`sys_ipc_send` hardcodes correlation 0, which is not true: `send.rs` overrides
it with `redirect_token.unwrap_or(correlation)`, so a send can carry a nonzero
token. The property that holds instead is that a process can only obtain a
token from a call addressed to it, which is a fact about the `pending_reply`
table rather than about this filter. It is proven in `ReplyAuthorization.lean`,
and the two together are what rule out a forged reply.
-/

namespace Nonos.ReplyCorrelation

/-- A message sitting in a reply inbox carries a correlation token. A genuine
    reply is stamped with the caller's call token. A `mk_ipc_send` carries 0
    unless the redirect path supplied a token, which it does only for a call
    addressed to the sender: see `ReplyAuthorization.lean`. -/
structure Msg where
  correlation : Nat

/-- Call tokens are drawn nonzero (`next_call_token`). -/
def ValidTok (t : Nat) : Prop := 0 < t

/-- The client delivery filter: a message is delivered to a call only when its
    correlation equals the call's token. -/
def delivered (callTok : Nat) (m : Msg) : Bool := decide (m.correlation = callTok)

/-- Delivery happens exactly when the correlation matches the token: there is no
    other accepting case. -/
theorem delivered_iff (callTok : Nat) (m : Msg) :
    delivered callTok m = true ↔ m.correlation = callTok := by
  unfold delivered; simp

/-- The genuine reply, stamped with the call token, is delivered. -/
theorem genuine_delivered (callTok : Nat) : delivered callTok ⟨callTok⟩ = true := by
  unfold delivered; simp

/-- A forged message (correlation 0) is never delivered to a call with a nonzero
    token: the injection cannot match. -/
theorem forged_never_delivered (callTok : Nat) (h : ValidTok callTok) :
    delivered callTok ⟨0⟩ = false := by
  unfold delivered ValidTok at *
  simp only [decide_eq_false_iff_not]
  omega

/-- Scanning the inbox: return the first message whose correlation matches the
    token, dropping the rest (`recv_reply_correlated` drains and filters). -/
def firstMatch (callTok : Nat) : List Msg → Option Msg
  | [] => none
  | m :: rest => if delivered callTok m = true then some m else firstMatch callTok rest

/-- Whatever the scan returns carries the matching correlation, so the caller
    only ever receives a reply to its own call. -/
theorem firstMatch_matches (callTok : Nat) :
    ∀ q m, firstMatch callTok q = some m → m.correlation = callTok := by
  intro q
  induction q with
  | nil => intro m h; simp [firstMatch] at h
  | cons a rest ih =>
    intro m h
    unfold firstMatch at h
    by_cases hd : delivered callTok a = true
    · rw [if_pos hd] at h; injection h with h'; rw [← h']
      exact (delivered_iff callTok a).mp hd
    · rw [if_neg hd] at h; exact ih m h

/-- An inbox containing only forged messages (all correlation 0) yields nothing
    for a nonzero call token: no injected message is ever returned. -/
theorem all_forged_none (callTok : Nat) (h : ValidTok callTok) :
    ∀ q, (∀ m ∈ q, m.correlation = 0) → firstMatch callTok q = none := by
  intro q
  induction q with
  | nil => intro _; rfl
  | cons a rest ih =>
    intro hall
    have ha : a.correlation = 0 := hall a (by simp)
    have hd : delivered callTok a = false := by
      have : a.correlation ≠ callTok := by unfold ValidTok at h; omega
      unfold delivered; simp [this]
    unfold firstMatch
    rw [if_neg (by rw [hd]; simp)]
    exact ih (fun m hm => hall m (by simp [hm]))

end Nonos.ReplyCorrelation
