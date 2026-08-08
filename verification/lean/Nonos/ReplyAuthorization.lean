/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Who is allowed to hold a caller's reply token.

`ReplyCorrelation.lean` proves the client filter: a reply is delivered only if
its correlation equals the token that call registered. That argument rests on
a premise stated there as "a message injected via `mk_ipc_send` carries
correlation 0", and that premise is not unconditionally true.
`src/syscall/microkernel/ipc/send.rs` overrides the correlation with
`redirect_token.unwrap_or(correlation)`, so a `sys_ipc_send` can carry a
nonzero token.

What actually holds is narrower and lives in `pending_reply`. The redirect
fires only when a process sends to its own reply endpoint, and the token comes
from `pop(sender_pid)`, keyed by the sender's own pid. The direct reply path is
the same shape: `reply.rs` calls `remove(caller_pid, &dest)` with
`current_pid()` as the key. Both readers key the table by themselves, and the
only writer is `sys_ipc_call`, which keys it by the server the caller's
endpoint resolved to.

So a process can obtain a token only from a call that was addressed to it.
That is what stops a confused deputy: without it, any capsule could harvest a
live token and land a forged reply inside another capsule's `mk_ipc_call`,
against a service that capsule trusts, leaving nothing behind to notice. This
file states that invariant over the table operations as written, and the
consumption property that goes with it, since a token reusable twice would let
one call's reply satisfy another.
-/

namespace Nonos.ReplyAuthorization

abbrev Pid := Nat
abbrev Inbox := Nat
abbrev Token := Nat

/-- A pending-reply entry: the caller, its private reply inbox, and the token
    that call registered. -/
structure Entry where
  caller : Pid
  inbox : Inbox
  token : Token
  deriving DecidableEq, Repr

/-- `PENDING`, flattened: entries paired with the server pid they are keyed
    under. The real table is a map to per-server queues; what the theorems need
    is the key and the order within a key, both of which this preserves. -/
abbrev Table := List (Pid × Entry)

/-- `push`: `sys_ipc_call` files the entry under the server its endpoint
    resolved to. This is the only writer. -/
def push (server : Pid) (e : Entry) (tbl : Table) : Table := tbl ++ [(server, e)]

/-- `remove`: the direct `mk_ipc_reply` path, matching on the caller's inbox
    under the replier's own key. Returns the token and the table without that
    entry. -/
def remove (server : Pid) (ib : Inbox) : Table → Option (Token × Table)
  | [] => none
  | (k, e) :: rest =>
    if k = server ∧ e.inbox = ib then some (e.token, rest)
    else
      match remove server ib rest with
      | some (t, rest') => some (t, (k, e) :: rest')
      | none => none

/-- `pop`: the redirect path, taking the oldest entry under the sender's own
    key. -/
def pop (server : Pid) : Table → Option (Entry × Table)
  | [] => none
  | (k, e) :: rest =>
    if k = server then some (e, rest)
    else
      match pop server rest with
      | some (x, rest') => some (x, (k, e) :: rest')
      | none => none

/-! ### A token can only come from an entry filed under the taker -/

/-- Whatever `remove` returns was an entry filed under the key it was asked
    for. Since `reply.rs` passes its own pid as that key, a process cannot
    reach another process's pending calls. -/
theorem remove_returns_an_entry_under_its_key (server : Pid) (ib : Inbox) :
    ∀ (tbl : Table) (t : Token) (tbl' : Table),
      remove server ib tbl = some (t, tbl') →
        ∃ e, (server, e) ∈ tbl ∧ e.token = t ∧ e.inbox = ib := by
  intro tbl
  induction tbl with
  | nil => intro t tbl' h; exact absurd h (by simp [remove])
  | cons hd rest ih =>
    intro t tbl' h
    obtain ⟨k, e⟩ := hd
    by_cases hm : k = server ∧ e.inbox = ib
    · simp [remove, hm] at h
      obtain ⟨ht, _⟩ := h
      exact ⟨e, by simp [hm.1], ht, hm.2⟩
    · simp [remove, hm] at h
      cases hr : remove server ib rest with
      | none => rw [hr] at h; exact absurd h (by simp)
      | some p =>
        obtain ⟨t2, rest2⟩ := p
        rw [hr] at h
        simp at h
        obtain ⟨e2, hmem, htok, hib⟩ := ih t2 rest2 hr
        exact ⟨e2, by simp [hmem], by rw [htok, h.1], hib⟩

/-- The same for `pop`, which the redirect path calls with the sender's own
    pid. -/
theorem pop_returns_an_entry_under_its_key (server : Pid) :
    ∀ (tbl : Table) (e : Entry) (tbl' : Table),
      pop server tbl = some (e, tbl') → (server, e) ∈ tbl := by
  intro tbl
  induction tbl with
  | nil => intro e tbl' h; exact absurd h (by simp [pop])
  | cons hd rest ih =>
    intro e tbl' h
    obtain ⟨k, e0⟩ := hd
    by_cases hm : k = server
    · simp [pop, hm] at h
      obtain ⟨he, _⟩ := h
      subst hm
      rw [← he]
      simp
    · simp [pop, hm] at h
      cases hr : pop server rest with
      | none => rw [hr] at h; exact absurd h (by simp)
      | some p =>
        obtain ⟨e2, rest2⟩ := p
        rw [hr] at h
        simp at h
        have := ih e2 rest2 hr
        rw [← h.1]
        simp [this]

/-! ### Which means the call was addressed to the taker -/

/-- The table is well formed against a record of which calls were made: every
    entry keyed under `s` records a call from `e.caller` to `s` carrying
    `e.token`. `push` is the only writer and files under the resolved server,
    so this is what `sys_ipc_call` establishes. -/
def WellFormed (calls : Pid → Pid → Token → Prop) (tbl : Table) : Prop :=
  ∀ s e, (s, e) ∈ tbl → calls e.caller s e.token

/-- Filing a call preserves well formedness, provided the call really was made
    to that server. -/
theorem push_preserves_wellformed (calls : Pid → Pid → Token → Prop)
    (tbl : Table) (server : Pid) (e : Entry)
    (hwf : WellFormed calls tbl) (hcall : calls e.caller server e.token) :
    WellFormed calls (push server e tbl) := by
  intro s e' hmem
  simp [push] at hmem
  rcases hmem with hmem | hmem
  · exact hwf s e' hmem
  · obtain ⟨hs, he⟩ := hmem
    subst hs
    subst he
    exact hcall

/-- **The reply authorization property.** A process that obtains a token
    through the direct reply path obtains it from a call that was addressed to
    that process.

    False the moment `remove` is keyed by anything other than the replier: key
    it by the destination pid instead, as would be the natural mistake, and any
    capsule could take a token belonging to a call it was never party to. -/
theorem a_reply_token_comes_from_a_call_to_the_replier
    (calls : Pid → Pid → Token → Prop) (tbl : Table) (replier : Pid) (ib : Inbox)
    (t : Token) (tbl' : Table) (hwf : WellFormed calls tbl)
    (h : remove replier ib tbl = some (t, tbl')) :
    ∃ caller, calls caller replier t := by
  obtain ⟨e, hmem, htok, _⟩ := remove_returns_an_entry_under_its_key replier ib tbl t tbl' h
  refine ⟨e.caller, ?_⟩
  rw [← htok]
  exact hwf replier e hmem

/-- The same for the redirect path, which is the one that lets a `mk_ipc_send`
    carry a nonzero correlation at all. -/
theorem a_redirect_token_comes_from_a_call_to_the_sender
    (calls : Pid → Pid → Token → Prop) (tbl : Table) (sender : Pid)
    (e : Entry) (tbl' : Table) (hwf : WellFormed calls tbl)
    (h : pop sender tbl = some (e, tbl')) :
    calls e.caller sender e.token :=
  hwf sender e (pop_returns_an_entry_under_its_key sender tbl e tbl' h)

/-! ### And it is spent when it is used -/

/-- `remove` shortens the table by exactly the entry it returned, so the same
    pending call cannot authorize two replies. -/
theorem remove_consumes_one (server : Pid) (ib : Inbox) :
    ∀ (tbl : Table) (t : Token) (tbl' : Table),
      remove server ib tbl = some (t, tbl') → tbl'.length + 1 = tbl.length := by
  intro tbl
  induction tbl with
  | nil => intro t tbl' h; exact absurd h (by simp [remove])
  | cons hd rest ih =>
    intro t tbl' h
    obtain ⟨k, e⟩ := hd
    by_cases hm : k = server ∧ e.inbox = ib
    · simp [remove, hm] at h
      obtain ⟨_, hrest⟩ := h
      rw [← hrest]
      simp
    · simp [remove, hm] at h
      cases hr : remove server ib rest with
      | none => rw [hr] at h; exact absurd h (by simp)
      | some p =>
        obtain ⟨t2, rest2⟩ := p
        rw [hr] at h
        simp at h
        have := ih t2 rest2 hr
        rw [← h.2]
        simp [← this]

/-- `pop` likewise. -/
theorem pop_consumes_one (server : Pid) :
    ∀ (tbl : Table) (e : Entry) (tbl' : Table),
      pop server tbl = some (e, tbl') → tbl'.length + 1 = tbl.length := by
  intro tbl
  induction tbl with
  | nil => intro e tbl' h; exact absurd h (by simp [pop])
  | cons hd rest ih =>
    intro e tbl' h
    obtain ⟨k, e0⟩ := hd
    by_cases hm : k = server
    · simp [pop, hm] at h
      obtain ⟨_, hrest⟩ := h
      rw [← hrest]
      simp
    · simp [pop, hm] at h
      cases hr : pop server rest with
      | none => rw [hr] at h; exact absurd h (by simp)
      | some p =>
        obtain ⟨e2, rest2⟩ := p
        rw [hr] at h
        simp at h
        have := ih e2 rest2 hr
        rw [← h.2]
        simp [← this]

/-- An empty table hands out nothing, so a capsule with no calls outstanding
    against it cannot stamp a reply at all. -/
theorem nothing_pending_yields_no_token (server : Pid) (ib : Inbox) :
    remove server ib [] = none := rfl

end Nonos.ReplyAuthorization
