/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

DHCP client state machine of capsule_net_dhcp. The implementation diverges
from RFC 2131 on purpose: its states are Init, Selecting, Requesting, Bound
and Renewing only; there is no Rebinding state and the capsule runs no T1/T2
or lease-expiry timers of its own (the caller drives renewal through
OP_LEASE_RENEW, so no timer ordering can be proved from the code). The DORA
driver (dora/acquire.rs) moves Init to Selecting on DISCOVER, Selecting to
Requesting on a matching OFFER, and Requesting to Bound on a matching ACK
with a nonzero yiaddr; every DORA failure resets to Init. wait_reply.rs drops
any frame whose xid differs from the outstanding request, so mismatched
offers, acks and naks are invisible. The renew handler moves Bound to
Renewing and back to Bound on ACK, collapses to Init on NAK, but on timeout
leaves the state at Renewing (another divergence, modeled as the code is).
Release resets to Init unconditionally. The theorems below show Bound is only
entered through a matching ACK from Requesting or Renewing, a whole trace
into Bound must contain such an ACK, wrong-xid frames never change state, NAK
collapses to Init, and release restores Init from every state.
-/

namespace Nonos.Dhcp

/-- Client states of dhcp/client_state.rs: no Rebinding exists in the code. -/
inductive St where
  | init
  | selecting
  | requesting
  | bound
  | renewing
deriving DecidableEq

/-- The machine: current state plus the outstanding transaction id, the one
    handed out by `STATE.next_xid()` when the request in flight was built. -/
structure M where
  st : St
  xid : Nat
deriving DecidableEq

/-- Events. `discover` and `renew` carry the fresh xid drawn for the exchange;
    received frames carry the xid seen on the wire. `timeout` is the exhausted
    poll budget of wait_reply.rs, `release` is OP_LEASE_RELEASE. -/
inductive Ev where
  | discover (x : Nat)
  | offer (x : Nat)
  | ack (x : Nat)
  | nak (x : Nat)
  | timeout
  | renew (x : Nat)
  | release

/-- One transition, exactly as the Rust writes the state word.
    dora/acquire.rs:37,39,54,59; wait_reply.rs:47 (xid filter);
    lease_renew.rs:43,57,77 (timeout leaves Renewing);
    lease_release.rs:47 (unconditional Init). -/
def step (m : M) : Ev → M
  | Ev.discover x => ⟨St.selecting, x⟩
  | Ev.offer x =>
      if m.st = St.selecting ∧ x = m.xid then ⟨St.requesting, m.xid⟩ else m
  | Ev.ack x =>
      if x = m.xid then
        match m.st with
        | St.requesting => ⟨St.bound, m.xid⟩
        | St.renewing => ⟨St.bound, m.xid⟩
        | _ => m
      else m
  | Ev.nak x =>
      if x = m.xid then
        match m.st with
        | St.requesting => ⟨St.init, m.xid⟩
        | St.renewing => ⟨St.init, m.xid⟩
        | _ => m
      else m
  | Ev.timeout =>
      match m.st with
      | St.selecting => ⟨St.init, m.xid⟩
      | St.requesting => ⟨St.init, m.xid⟩
      | _ => m
  | Ev.renew x => if m.st = St.bound then ⟨St.renewing, x⟩ else m
  | Ev.release => ⟨St.init, m.xid⟩

/-- Run a trace of events. -/
def run (m : M) : List Ev → M
  | [] => m
  | e :: es => run (step m e) es

/-- Bound is entered only by an ACK whose xid matches the outstanding request,
    and only from Requesting (DORA) or Renewing (lease renewal). -/
theorem bound_only_via_matching_ack (m : M) (e : Ev)
    (h0 : m.st ≠ St.bound) (h : (step m e).st = St.bound) :
    e = Ev.ack m.xid ∧ (m.st = St.requesting ∨ m.st = St.renewing) := by
  obtain ⟨st, xid⟩ := m
  cases e <;> simp only [step] at h <;>
    (try split at h) <;> (try split at h) <;> simp_all

/-- Any trace that starts off Bound and ends Bound contains an ACK event:
    no sequence of offers, naks, timeouts, renews or releases fabricates a
    lease. -/
theorem bound_trace_has_ack (m : M) (es : List Ev)
    (h0 : m.st ≠ St.bound) (h : (run m es).st = St.bound) :
    ∃ x, Ev.ack x ∈ es := by
  induction es generalizing m with
  | nil => exact absurd h h0
  | cons e es ih =>
    by_cases hb : (step m e).st = St.bound
    · exact ⟨m.xid, ((bound_only_via_matching_ack m e h0 hb).1 ▸ List.mem_cons_self _ _)⟩
    · obtain ⟨x, hx⟩ := ih (step m e) hb h
      exact ⟨x, List.mem_cons_of_mem _ hx⟩

/-- An OFFER carrying the wrong xid never changes the machine: wait_reply.rs
    drops it before the DORA driver ever sees it. -/
theorem offer_wrong_xid_ignored (m : M) (x : Nat) (h : x ≠ m.xid) :
    step m (Ev.offer x) = m := by
  simp [step, h]

/-- An ACK carrying the wrong xid is equally invisible. -/
theorem ack_wrong_xid_ignored (m : M) (x : Nat) (h : x ≠ m.xid) :
    step m (Ev.ack x) = m := by
  simp [step, h]

/-- A NAK carrying the wrong xid is equally invisible. -/
theorem nak_wrong_xid_ignored (m : M) (x : Nat) (h : x ≠ m.xid) :
    step m (Ev.nak x) = m := by
  simp [step, h]

/-- A matching NAK either collapses the machine to Init (from Requesting or
    Renewing, the only states with a request in flight) or leaves it alone:
    a NAK never leaves a half-bound lease behind. -/
theorem nak_resets_or_ignored (m : M) (x : Nat) :
    (step m (Ev.nak x)).st = St.init ∨ step m (Ev.nak x) = m := by
  by_cases hx : x = m.xid
  · cases hs : m.st <;> simp_all [step]
  · exact Or.inr (nak_wrong_xid_ignored m x hx)

/-- A matching NAK during DORA's Requesting phase lands in Init
    (acquire.rs reset). -/
theorem nak_requesting_init (m : M) (h : m.st = St.requesting) :
    (step m (Ev.nak m.xid)).st = St.init := by
  simp [step, h]

/-- A matching NAK during renewal lands in Init (lease_renew.rs:57). -/
theorem nak_renewing_init (m : M) (h : m.st = St.renewing) :
    (step m (Ev.nak m.xid)).st = St.init := by
  simp [step, h]

/-- A DORA timeout resets Selecting and Requesting to Init, the acquire.rs
    reset path for every wait failure. -/
theorem timeout_dora_resets (m : M)
    (h : m.st = St.selecting ∨ m.st = St.requesting) :
    (step m Ev.timeout).st = St.init := by
  cases h with
  | inl h => simp [step, h]
  | inr h => simp [step, h]

/-- Divergence from RFC 2131, proved as written: a renewal timeout leaves the
    machine in Renewing (lease_renew.rs answers E_TIMEOUT without touching the
    state word); there is no T2/Rebinding fallback. -/
theorem timeout_renewing_stays (m : M) (h : m.st = St.renewing) :
    step m Ev.timeout = m := by
  cases m; simp_all [step]

/-- Release restores Init from every state; with no in-capsule lease-expiry
    timer this is the code's only unconditional way back to the ground state
    (lease_release.rs:47). -/
theorem release_resets (m : M) : (step m Ev.release).st = St.init := by
  simp [step]

/-- Bound keeps the xid of the ACK'd exchange: the lease is tied to the very
    transaction that produced it. -/
theorem bound_keeps_xid (m : M) (e : Ev)
    (h0 : m.st ≠ St.bound) (h : (step m e).st = St.bound) :
    (step m e).xid = m.xid := by
  obtain ⟨st, xid⟩ := m
  cases e <;> simp only [step] at h ⊢ <;>
    (try split at h) <;> (try split at h) <;> simp_all

end Nonos.Dhcp
