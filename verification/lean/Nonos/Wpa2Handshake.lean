/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The WPA2 four-way handshake as the WiFi supplicant runs it
(`userland/nonos_wifi_core/src/wpa/supplicant/{state,step}.rs`). The SNonce is
fixed when the handshake starts; message 1 delivers the ANonce and the PTK is
derived from exactly that nonce pair; message 3 must carry a MIC that verifies
under the derived key and repeat the ANonce, and only then is the group key
installed and the state advanced to Connected; any verification failure lands
in Failed, which no message leaves. EAPOL replay counters must be strictly
increasing, so a captured frame played back never moves the machine. The MIC
itself is bounded to the announced EAPOL length (`eapol/mic.rs::verify_mic`):
a received MPDU can carry a trailing FCS past the EAPOL-Key frame, and a
historical bug hashed the MIC over those trailer bytes, rejecting every valid
message 3; the fix hashes `frame[..HEADER_LEN + key_data_len]` only. The
theorems below show keys install only on a MIC-valid message 3 with the bound
ANonce, a non-increasing replay counter never changes the state, the MIC input
never reaches past the EAPOL length (while the buggy whole-buffer input always
did when a trailer was present), and a completed handshake derived its PTK
from the ANonce and the SNonce that were fixed before derivation.
-/

namespace Nonos.Wpa2Handshake

/-- Where the handshake stands, mirroring `supplicant::State`. -/
inductive Phase
  | start
  | ptkDerived
  | connected
  | failed
  deriving DecidableEq

/-- An incoming pairwise EAPOL-Key message: whether the MIC bit is set, whether
    the carried MIC verifies under the KCK, the replay counter and the nonce
    field. Message 1 has no MIC; message 3 has one. -/
structure Msg where
  hasMic : Bool
  micValid : Bool
  replay : Nat
  nonce : Nat

/-- The supplicant: its phase, the SNonce fixed at start, the ANonce learned
    from message 1, the last replay counter seen, the PTK recorded as the nonce
    pair it was derived from, and whether keys are installed. -/
structure Supp where
  phase : Phase
  snonce : Nat
  anonce : Nat
  lastReplay : Nat
  ptk : Option (Nat × Nat)
  installed : Bool

/-- Begin a handshake: the SNonce is fixed now, before any message arrives. -/
def init (snonce : Nat) : Supp :=
  { phase := .start, snonce := snonce, anonce := 0, lastReplay := 0,
    ptk := none, installed := false }

/-- Drive one message through the handshake, as `Supplicant::step` does. A
    non-increasing replay counter is dropped outright. In Start a MIC-less
    message 1 fixes the ANonce and derives the PTK from (ANonce, SNonce). In
    PtkDerived a MIC-carrying message 3 advances to Connected and installs keys
    only if its MIC verifies and its nonce repeats the bound ANonce; otherwise
    the handshake is abandoned in Failed. Everything else is ignored. -/
def step (s : Supp) (m : Msg) : Supp :=
  if m.replay ≤ s.lastReplay then s
  else
    match s.phase, m.hasMic with
    | .start, false =>
        { s with phase := .ptkDerived, anonce := m.nonce,
                 ptk := some (m.nonce, s.snonce), lastReplay := m.replay }
    | .ptkDerived, true =>
        if m.micValid = true ∧ m.nonce = s.anonce then
          { s with phase := .connected, installed := true,
                   lastReplay := m.replay }
        else
          { s with phase := .failed }
    | _, _ => s

/-- Run the supplicant over a message sequence. -/
def run (s : Supp) : List Msg → Supp
  | [] => s
  | m :: ms => run (step s m) ms

/-- A replayed message, one whose counter has not strictly increased, changes
    nothing at all: the state after it is the state before it. -/
theorem replay_never_advances (s : Supp) (m : Msg)
    (h : m.replay ≤ s.lastReplay) : step s m = s := by
  simp [step, h]

/-- The replay counter floor never falls, so a frame captured now stays dead
    for the rest of the handshake. -/
theorem lastReplay_monotone (s : Supp) (m : Msg) :
    s.lastReplay ≤ (step s m).lastReplay := by
  obtain ⟨ph, sn, an, lr, pk, ins⟩ := s
  obtain ⟨mic, mv, mr, mn⟩ := m
  by_cases hr : mr ≤ lr
  · simp [step, hr]
  · by_cases hc : mv = true ∧ mn = an <;>
      cases ph <;> cases mic <;>
      simp [step, hr, hc] <;> omega

/-- No key install without a MIC-valid message 3: if keys were not installed
    and one step installs them, the machine was awaiting message 3, the message
    carried a MIC, that MIC verified, the nonce repeated the bound ANonce, and
    the replay counter strictly increased. -/
theorem install_requires_valid_msg3 (s : Supp) (m : Msg)
    (h0 : s.installed = false) (h : (step s m).installed = true) :
    s.phase = .ptkDerived ∧ m.hasMic = true ∧ m.micValid = true ∧
      m.nonce = s.anonce ∧ s.lastReplay < m.replay := by
  obtain ⟨ph, sn, an, lr, pk, ins⟩ := s
  obtain ⟨mic, mv, mr, mn⟩ := m
  by_cases hr : mr ≤ lr
  · simp [replay_never_advances ⟨ph, sn, an, lr, pk, ins⟩ ⟨mic, mv, mr, mn⟩ hr, h0] at h
  · by_cases hc : mv = true ∧ mn = an <;>
      cases ph <;> cases mic <;>
      simp [step, hr, hc, h0] at h ⊢ <;> omega

/-- A message whose MIC does not verify never installs keys, whatever else it
    claims. -/
theorem bad_mic_never_installs (s : Supp) (m : Msg)
    (hv : m.micValid = false) : (step s m).installed = s.installed := by
  obtain ⟨ph, sn, an, lr, pk, ins⟩ := s
  obtain ⟨mic, mv, mr, mn⟩ := m
  by_cases hr : mr ≤ lr
  · simp [replay_never_advances ⟨ph, sn, an, lr, pk, ins⟩ ⟨mic, mv, mr, mn⟩ hr]
  · have hc : ¬(mv = true ∧ mn = an) := by
      intro hcc; rw [hcc.1] at hv; simp at hv
    cases ph <;> cases mic <;> simp [step, hr, hc]

/-- Failed absorbs: once a frame fails verification the handshake is abandoned
    and no later message resurrects it. -/
theorem failed_is_absorbing (s : Supp) (m : Msg) (h : s.phase = .failed) :
    step s m = s := by
  by_cases hr : m.replay ≤ s.lastReplay
  · exact replay_never_advances s m hr
  · cases hm : m.hasMic <;> simp [step, hr, h, hm]

/-- The SNonce is never rewritten by a message: it stays the one fixed when
    the handshake began. -/
theorem step_keeps_snonce (s : Supp) (m : Msg) : (step s m).snonce = s.snonce := by
  obtain ⟨ph, sn, an, lr, pk, ins⟩ := s
  obtain ⟨mic, mv, mr, mn⟩ := m
  by_cases hr : mr ≤ lr
  · simp [step, hr]
  · by_cases hc : mv = true ∧ mn = an <;>
      cases ph <;> cases mic <;>
      simp [step, hr, hc]

/-- Over any run the SNonce is still the one chosen at init. -/
theorem run_keeps_snonce (s : Supp) (ms : List Msg) :
    (run s ms).snonce = s.snonce := by
  induction ms generalizing s with
  | nil => rfl
  | cons m ms ih => rw [run, ih, step_keeps_snonce]

/-- The PTK changes only on message 1 in Start, and then it is derived from
    exactly the message's ANonce and the already-fixed SNonce: both nonces are
    pinned before the key exists. -/
theorem ptk_derived_only_from_start (s : Supp) (m : Msg)
    (h : (step s m).ptk ≠ s.ptk) :
    s.phase = .start ∧ m.hasMic = false ∧
      (step s m).ptk = some (m.nonce, s.snonce) := by
  obtain ⟨ph, sn, an, lr, pk, ins⟩ := s
  obtain ⟨mic, mv, mr, mn⟩ := m
  by_cases hr : mr ≤ lr
  · rw [replay_never_advances ⟨ph, sn, an, lr, pk, ins⟩ ⟨mic, mv, mr, mn⟩ hr] at h; exact absurd rfl h
  · by_cases hc : mv = true ∧ mn = an <;>
      cases ph <;> cases mic <;>
      simp [step, hr, hc] at h ⊢

/-- The handshake invariant: once the PTK exists (PtkDerived or Connected) it
    is the key derived from the stored ANonce and the stored SNonce. -/
def PtkBound (s : Supp) : Prop :=
  s.phase = .ptkDerived ∨ s.phase = .connected →
    s.ptk = some (s.anonce, s.snonce)

/-- The invariant holds at init: no PTK exists yet. -/
theorem init_ptkBound (snonce : Nat) : PtkBound (init snonce) := by
  simp [PtkBound, init]

/-- Every step preserves the invariant. -/
theorem step_preserves_ptkBound (s : Supp) (m : Msg) (h : PtkBound s) :
    PtkBound (step s m) := by
  obtain ⟨ph, sn, an, lr, pk, ins⟩ := s
  obtain ⟨mic, mv, mr, mn⟩ := m
  by_cases hr : mr ≤ lr
  · rw [replay_never_advances ⟨ph, sn, an, lr, pk, ins⟩ ⟨mic, mv, mr, mn⟩ hr]; exact h
  · by_cases hc : mv = true ∧ mn = an <;>
      cases ph <;> cases mic <;>
      simp [PtkBound, step, hr, hc] at h ⊢ <;>
      simp_all

/-- The invariant survives any run. -/
theorem run_preserves_ptkBound (s : Supp) (ms : List Msg) (h : PtkBound s) :
    PtkBound (run s ms) := by
  induction ms generalizing s with
  | nil => exact h
  | cons m ms ih => exact ih (step s m) (step_preserves_ptkBound s m h)

/-- Completion binds the nonces: if a run from a fresh handshake reaches
    Connected, the PTK is exactly the key derived from the ANonce the machine
    holds and the SNonce fixed at init, so both nonces were pinned before the
    derivation the connection rests on. -/
theorem connected_ptk_from_fixed_nonces (snonce : Nat) (ms : List Msg)
    (h : (run (init snonce) ms).phase = .connected) :
    (run (init snonce) ms).ptk =
      some ((run (init snonce) ms).anonce, snonce) := by
  have hb := run_preserves_ptkBound (init snonce) ms (init_ptkBound snonce)
  have := hb (Or.inr h)
  rwa [run_keeps_snonce] at this

/-! ## MIC input bounding

`verify_mic` computes the MIC over `frame[..HEADER_LEN + key_data_len]`, the
exact EAPOL-Key frame the length field announces, refusing a frame whose
announced length runs past the buffer. The received buffer may extend further
(the radio's trailing FCS); those bytes are never hashed. The historical bug
hashed the whole buffer instead. -/

/-- The byte indices the fixed MIC covers: `[0, eapolLen)`. -/
def micIndices (eapolLen : Nat) : List Nat := List.range eapolLen

/-- The bounding check of `verify_mic`: the announced EAPOL length is accepted
    only when it fits inside the received buffer. -/
def micEnd (announced wireLen : Nat) : Option Nat :=
  if announced ≤ wireLen then some announced else none

/-- An accepted MIC bound never exceeds the received buffer: hashing stays in
    bounds. -/
theorem micEnd_within_buffer (announced wireLen e : Nat)
    (h : micEnd announced wireLen = some e) : e ≤ wireLen := by
  unfold micEnd at h
  split at h
  · cases h; assumption
  · cases h

/-- The accepted bound is exactly the announced EAPOL length, not the buffer
    length: the trailer never widens the MIC input. -/
theorem micEnd_is_announced (announced wireLen e : Nat)
    (h : micEnd announced wireLen = some e) : e = announced := by
  unfold micEnd at h
  split at h
  · cases h; rfl
  · cases h

/-- A frame whose announced length overruns the buffer is refused rather than
    hashed. -/
theorem micEnd_refuses_overrun (announced wireLen : Nat)
    (h : wireLen < announced) : micEnd announced wireLen = none := by
  simp [micEnd]; omega

/-- The MIC input never includes a byte at or past the EAPOL frame length:
    every index hashed is strictly inside the announced frame. -/
theorem mic_input_within_eapol (eapolLen i : Nat)
    (h : i ∈ micIndices eapolLen) : i < eapolLen := by
  simpa [micIndices] using h

/-- In particular no trailer byte is hashed: a byte of the FCS lives at index
    `eapolLen + k`, and no such index is in the MIC input. -/
theorem mic_input_excludes_fcs (eapolLen k : Nat) :
    eapolLen + k ∉ micIndices eapolLen := by
  simp [micIndices]

/-- The historical bug, by contrast, hashed the whole received buffer; with any
    trailer present that input contains the first FCS byte, so the buggy and
    fixed MIC inputs provably differ on every frame with an FCS. -/
theorem buggy_input_includes_fcs (eapolLen fcsLen : Nat) (h : 0 < fcsLen) :
    eapolLen ∈ micIndices (eapolLen + fcsLen) := by
  simp [micIndices]; omega

end Nonos.Wpa2Handshake
