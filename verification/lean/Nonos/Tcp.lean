/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

TCP connection machine of capsule_net_tcp, modeled as the code is written,
not as RFC 793 reads. The implementation has no Closed enum variant: a dead
connection is removed from the table (Reap), so `closed` here models the
absent entry, which no later segment revives; an in-window RST reaps at
once, without the RFC challenge-ACK. Per step_entry.rs, SynSent reaches
Established only on a SYN|ACK whose ack equals snd.nxt, SynReceived only on
an ACK of snd.nxt (a piggybacked FIN lands in CloseWait instead), and both
Established and the closing-side states gate every segment on
seq::acceptable, answering out-of-window traffic with a duplicate ACK and no
state change. Local close moves Established to FinWait1 and CloseWait to
LastAck; any other state is removed (reassembly, send.una and window updates
are not modeled). Sequence comparison is the wrapping u32 arithmetic of
tcp/seq.rs, proved over Nat with explicit modulus 2^32: the theorems cover
the handshake edges, the window gates, wraparound ordering and reaping.
-/

namespace Nonos.Tcp

/-- The u32 modulus and half range of tcp/seq.rs. -/
def M32 : Nat := 4294967296
def H32 : Nat := 2147483648

/-- u32 wrapping add and sub, for operands below the modulus. -/
def wadd (a b : Nat) : Nat := (a + b) % M32
def wsub (a b : Nat) : Nat := (a + M32 - b) % M32

/-- seq::lt — `(a.wrapping_sub(b) as i32) < 0` (seq.rs:17). -/
def sltb (a b : Nat) : Bool := decide (H32 ≤ wsub a b)

/-- seq::leq — `(a.wrapping_sub(b) as i32) <= 0` (seq.rs:21). -/
def sleqb (a b : Nat) : Bool := decide (wsub a b = 0 ∨ H32 ≤ wsub a b)

/-- seq::between — `leq lo x && lt x hi` (seq.rs:29). -/
def betweenb (x lo hi : Nat) : Bool := sleqb lo x && sltb x hi

/-- seq::acceptable, clause for clause (seq.rs:33). -/
def acceptable (sq len nxt wnd : Nat) : Bool :=
  if len = 0 then
    if wnd = 0 then decide (sq = nxt) else betweenb sq nxt (wadd nxt wnd)
  else if wnd = 0 then false
  else betweenb sq nxt (wadd nxt wnd)
    || betweenb (wsub (wadd sq len) 1) nxt (wadd nxt wnd)

/-- `lt` sees a forward distance under 2^31 even across wraparound, and
    never claims its reverse. -/
theorem slt_forward (a d : Nat) (h1 : 0 < d) (h2 : d < H32) :
    sltb a ((a + d) % M32) = true := by
  simp only [sltb, wsub, M32, H32, decide_eq_true_eq] at *; omega

theorem slt_backward (a d : Nat) (h1 : 0 < d) (h2 : d < H32) :
    sltb ((a + d) % M32) a = false := by
  simp only [sltb, wsub, M32, H32, decide_eq_false_iff_not] at *; omega

/-- `leq` accepts every forward distance under 2^31, including zero. -/
theorem sleq_forward (a d : Nat) (h2 : d < H32) :
    sleqb a ((a + d) % M32) = true := by
  simp only [sleqb, wsub, M32, H32, decide_eq_true_eq] at *; omega

/-- `lt` is irreflexive and `leq` reflexive at every sequence number. -/
theorem slt_irrefl (a : Nat) : sltb a a = false := by
  simp only [sltb, wsub, M32, H32, decide_eq_false_iff_not]; omega

theorem sleq_refl (a : Nat) : sleqb a a = true := by
  simp only [sleqb, wsub, M32, H32, decide_eq_true_eq]; omega

/-- Connection states of tcp/state.rs, plus `closed` for the reaped entry. -/
inductive St where
  | listen | synSent | synReceived | established | closeWait
  | finWait1 | finWait2 | closing | timeWait | lastAck | closed
deriving DecidableEq

/-- An incoming segment as step_entry.rs consumes it. -/
structure Seg where
  syn : Bool
  ack : Bool
  fin : Bool
  rst : Bool
  seq : Nat
  ackNo : Nat
  len : Nat

/-- The modeled connection block. -/
structure Conn where
  st : St
  sndNxt : Nat
  rcvNxt : Nat
  rcvWnd : Nat

/-- rst::in_window (rst.rs:19): `seq.wrapping_sub(rcv_nxt) < wnd.max(1)`. -/
def rstInWindow (c : Conn) (s : Seg) : Bool :=
  decide (wsub s.seq c.rcvNxt < max c.rcvWnd 1)

/-- Established receive (transitions/established/step.rs): payload starting
    at rcv.nxt advances it, a FIN at or before rcv.nxt enters CloseWait. -/
def rxEstablished (c : Conn) (s : Seg) : Conn :=
  if acceptable s.seq s.len c.rcvNxt c.rcvWnd then
    let c1 := if s.len ≠ 0 ∧ s.seq = c.rcvNxt then
      { c with rcvNxt := wadd c.rcvNxt s.len } else c
    if s.fin && sleqb s.seq c1.rcvNxt then
      { c1 with st := St.closeWait, rcvNxt := wadd c1.rcvNxt 1 }
    else c1
  else c

/-- Closing-side receive (transitions/closing.rs): FIN counts as one window
    unit, `acks_fin` is an ACK of snd.nxt, LastAck reaps on the final ACK. -/
def rxClosing (c : Conn) (s : Seg) : Conn :=
  if acceptable s.seq (if s.fin then 1 else 0) c.rcvNxt c.rcvWnd then
    let acksFin := s.ack && decide (s.ackNo = c.sndNxt)
    match c.st with
    | St.finWait1 =>
        if acksFin && s.fin then { c with st := St.timeWait, rcvNxt := wadd s.seq 1 }
        else if acksFin then { c with st := St.finWait2 }
        else if s.fin then { c with st := St.closing, rcvNxt := wadd s.seq 1 }
        else c
    | St.finWait2 => if s.fin then { c with st := St.timeWait, rcvNxt := wadd s.seq 1 } else c
    | St.closing => if acksFin then { c with st := St.timeWait } else c
    | St.timeWait => if s.fin then { c with rcvNxt := wadd s.seq 1 } else c
    | St.lastAck => if acksFin then { c with st := St.closed } else c
    | _ => c
  else c

/-- One received segment (tcp_rx/existing/step_entry.rs). Listen never
    matches a connection (lookup.rs:42) and a reaped entry is gone: inert. -/
def rx (c : Conn) (s : Seg) : Conn :=
  if c.st = St.closed ∨ c.st = St.listen then c
  else if s.rst then
    if rstInWindow c s then { c with st := St.closed } else c
  else match c.st with
    | St.synSent =>
        if s.syn && s.ack && decide (s.ackNo = c.sndNxt) then
          { c with st := St.established, rcvNxt := wadd s.seq 1 }
        else c
    | St.synReceived =>
        if s.ack && decide (s.ackNo = c.sndNxt) then
          if s.fin then { c with st := St.closeWait, rcvNxt := wadd c.rcvNxt 1 }
          else { c with st := St.established }
        else c
    | St.established => rxEstablished c s
    | _ => rxClosing c s

/-- Local close, handlers/close.rs:36: Established sends FIN and enters
    FinWait1, CloseWait enters LastAck, anything else is removed. -/
def closeStep (c : Conn) : Conn :=
  if c.st = St.established then { c with st := St.finWait1, sndNxt := wadd c.sndNxt 1 }
  else if c.st = St.closeWait then { c with st := St.lastAck, sndNxt := wadd c.sndNxt 1 }
  else { c with st := St.closed }

/-- Established is entered only along the two handshake edges the code has:
    SynSent on a SYN|ACK acking snd.nxt, or SynReceived on a plain ACK of
    snd.nxt without a piggybacked FIN. -/
theorem established_only_via_handshake (c : Conn) (s : Seg)
    (h0 : c.st ≠ St.established) (h : (rx c s).st = St.established) :
    s.rst = false ∧ s.ack = true ∧ s.ackNo = c.sndNxt ∧
      ((c.st = St.synSent ∧ s.syn = true) ∨ (c.st = St.synReceived ∧ s.fin = false)) := by
  obtain ⟨st, sn, rn, rw⟩ := c
  revert h
  cases st <;> simp only [rx, rxEstablished, rxClosing] <;>
    (repeat' split) <;> intro h <;> simp_all

/-- Local close never fabricates an Established connection. -/
theorem close_never_establishes (c : Conn) : (closeStep c).st ≠ St.established := by
  obtain ⟨st, sn, rn, rw⟩ := c
  cases st <;> simp [closeStep]

/-- A non-RST segment outside the receive window is never accepted in
    Established: the connection is unchanged, only a duplicate ACK goes out. -/
theorem out_of_window_ignored (c : Conn) (s : Seg)
    (hst : c.st = St.established) (hrst : s.rst = false)
    (hw : acceptable s.seq s.len c.rcvNxt c.rcvWnd = false) :
    rx c s = c := by
  obtain ⟨st, sn, rn, rw⟩ := c
  simp_all [rx, rxEstablished]

/-- The same window gate guards every closing-side state, with the FIN
    counted as one sequence unit exactly as closing.rs computes seg_len. -/
theorem out_of_window_closing_ignored (c : Conn) (s : Seg)
    (hst : c.st = St.finWait1 ∨ c.st = St.finWait2 ∨ c.st = St.closing ∨
      c.st = St.timeWait ∨ c.st = St.lastAck)
    (hrst : s.rst = false)
    (hw : acceptable s.seq (if s.fin then 1 else 0) c.rcvNxt c.rcvWnd = false) :
    rx c s = c := by
  obtain ⟨st, sn, rn, rw⟩ := c
  rcases hst with h | h | h | h | h <;> simp_all [rx, rxClosing]

/-- An in-window RST reaps any live non-listen connection on the spot, while
    the rst.rs gate drops an out-of-window RST without effect. -/
theorem rst_in_window_closes (c : Conn) (s : Seg)
    (hne : c.st ≠ St.closed) (hnl : c.st ≠ St.listen)
    (hrst : s.rst = true) (hw : rstInWindow c s = true) :
    (rx c s).st = St.closed := by
  obtain ⟨st, sn, rn, rw⟩ := c
  cases st <;> simp_all [rx]

theorem rst_out_of_window_ignored (c : Conn) (s : Seg)
    (hrst : s.rst = true) (hw : rstInWindow c s = false) :
    rx c s = c := by
  obtain ⟨st, sn, rn, rw⟩ := c
  cases st <;> simp_all [rx]

/-- Once reaped, always reaped: no segment revives a closed connection and a
    redundant local close leaves it closed. -/
theorem closed_absorbing_rx (c : Conn) (s : Seg) (h : c.st = St.closed) :
    rx c s = c := by
  obtain ⟨st, sn, rn, rw⟩ := c
  simp_all [rx]

theorem closed_absorbing_close (c : Conn) (h : c.st = St.closed) :
    (closeStep c).st = St.closed := by
  simp [closeStep, h]

end Nonos.Tcp
