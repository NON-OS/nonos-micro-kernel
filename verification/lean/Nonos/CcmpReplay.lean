/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

CCMP packet numbers, as the WiFi data plane handles them
(`userland/nonos_wifi_core/src/dot11/data.rs`, driver counter in
`capsule_driver_iwlwifi/src/server/handlers/wifi_data.rs`). Every protected
frame carries a 48-bit packet number: the transmitter increments its PN before
each frame and the PN is packed into the 8-octet CCMP header (`ccmp_header`,
PN0/PN1 then PN2..PN5 around the ExtIV byte) and into the AES-CCM nonce
(`build_nonce`), so under one temporal key a nonce is exactly a PN. The
receiver recovers the PN (`pn_from_header`) and must drop any frame whose PN
does not strictly exceed the last accepted one, the CCMP replay window. The
theorems below show the header packing is lossless over the whole 48-bit
range, so distinct PNs never collide in a nonce; the transmitter's PNs are
strictly increasing and refuse to wrap at 2^48, so no CCM nonce is ever reused
under a key and exhaustion forces a rekey; a replayed or reordered-stale frame
never changes the receiver state; and once a PN is accepted it can never be
accepted again, however many frames follow.
-/

namespace Nonos.CcmpReplay

/-- The 48-bit PN space: `2 ^ 48`. -/
def pnLimit : Nat := 281474976710656

theorem pnLimit_eq : pnLimit = 2 ^ 48 := by decide

/-! ## Header packing

`ccmp_header` splits the PN into six octets (PN0 least significant) and
`pn_from_header` reassembles them; `build_nonce` packs the same six octets big
endian. Both are faithful exactly because the split is lossless below 2^48. -/

/-- The six PN octets of the CCMP header, least significant first, as
    `ccmp_header` emits them. -/
structure Hdr where
  pn0 : Nat
  pn1 : Nat
  pn2 : Nat
  pn3 : Nat
  pn4 : Nat
  pn5 : Nat
  deriving DecidableEq

/-- Pack a PN into its six header octets (`ccmp_header` / `build_nonce`). -/
def encode (pn : Nat) : Hdr :=
  ⟨pn % 256, pn / 256 % 256, pn / 65536 % 256, pn / 16777216 % 256,
   pn / 4294967296 % 256, pn / 1099511627776 % 256⟩

/-- Recover the PN from the six octets (`pn_from_header`). -/
def decode (h : Hdr) : Nat :=
  h.pn0 + 256 * h.pn1 + 65536 * h.pn2 + 16777216 * h.pn3 +
    4294967296 * h.pn4 + 1099511627776 * h.pn5

/-- The packing is lossless over the whole 48-bit range: the receiver sees
    exactly the PN the transmitter stamped. -/
theorem decode_encode (pn : Nat) (h : pn < pnLimit) :
    decode (encode pn) = pn := by
  simp only [encode, decode, pnLimit] at *
  omega

/-- Distinct 48-bit PNs pack to distinct headers, hence distinct CCM nonces
    under one key: the nonce space never collides. -/
theorem encode_injective (p q : Nat) (hp : p < pnLimit) (hq : q < pnLimit)
    (h : encode p = encode q) : p = q := by
  have := congrArg decode h
  rwa [decode_encode p hp, decode_encode q hq] at this

/-! ## Transmit side

The driver keeps one PN counter per key and increments it before every
protected frame, so each frame's nonce is fresh. At the top of the 48-bit
space the counter must not wrap: PN 0 would repeat a nonce already used under
this key, so exhaustion refuses to send until a rekey resets the counter. -/

/-- The transmitter: the last PN stamped under the current key. -/
structure Tx where
  pn : Nat

/-- Send one frame: advance the PN and stamp it, refusing to wrap past the
    48-bit space. `none` means the key is exhausted and a rekey is required. -/
def send (t : Tx) : Option (Nat × Tx) :=
  if t.pn + 1 < pnLimit then some (t.pn + 1, ⟨t.pn + 1⟩) else none

/-- A sent frame carries a PN strictly above every earlier one under this
    key. -/
theorem send_strictly_increases (t : Tx) (p : Nat) (t' : Tx)
    (h : send t = some (p, t')) : t.pn < p ∧ t'.pn = p := by
  unfold send at h
  split at h
  · cases h; exact ⟨Nat.lt_succ_self _, rfl⟩
  · cases h

/-- Every stamped PN stays inside the 48-bit field the header can carry. -/
theorem send_within_field (t : Tx) (p : Nat) (t' : Tx)
    (h : send t = some (p, t')) : p < pnLimit := by
  unfold send at h
  split at h
  · cases h; assumption
  · cases h

/-- Two successive frames never share a PN, so their CCM nonces differ: no
    keystream reuse under one temporal key. -/
theorem no_nonce_reuse (t : Tx) (p₁ p₂ : Nat) (t₁ t₂ : Tx)
    (h₁ : send t = some (p₁, t₁)) (h₂ : send t₁ = some (p₂, t₂)) :
    encode p₁ ≠ encode p₂ := by
  intro he
  have hb₁ := send_within_field t p₁ t₁ h₁
  have hb₂ := send_within_field t₁ p₂ t₂ h₂
  have hi₁ := send_strictly_increases t p₁ t₁ h₁
  have hi₂ := send_strictly_increases t₁ p₂ t₂ h₂
  have := encode_injective p₁ p₂ hb₁ hb₂ he
  omega

/-- At the top of the PN space the transmitter refuses to send: wrapping to a
    reused nonce is impossible, a rekey is forced instead. -/
theorem exhaustion_forces_rekey (t : Tx) (h : pnLimit ≤ t.pn + 1) :
    send t = none := by
  simp [send]; omega

/-! ## Receive side: the replay window

The receiver tracks the highest PN accepted under the key and delivers a frame
only when its PN strictly exceeds it. A replayed frame, or a stale reordered
one, is dropped without touching the state. -/

/-- The receiver: the highest PN accepted so far. -/
structure Rx where
  lastPn : Nat

/-- A frame is fresh when its PN strictly exceeds the highest accepted one. -/
def Fresh (r : Rx) (pn : Nat) : Prop := r.lastPn < pn

/-- Process a frame: accept and advance on a fresh PN, drop otherwise. -/
def accept (r : Rx) (pn : Nat) : Rx :=
  if r.lastPn < pn then ⟨pn⟩ else r

/-- A replayed frame, one whose PN does not exceed the window, changes nothing:
    the receiver state after it is the state before it. -/
theorem replay_dropped (r : Rx) (pn : Nat) (h : pn ≤ r.lastPn) :
    accept r pn = r := by
  simp [accept]; omega

/-- A replayed frame is not fresh, so it is never delivered. -/
theorem replay_not_fresh (r : Rx) (pn : Nat) (h : pn ≤ r.lastPn) :
    ¬ Fresh r pn := by
  simp [Fresh]; omega

/-- Accepting a fresh frame advances the window exactly to its PN. -/
theorem accept_advances (r : Rx) (pn : Nat) (h : Fresh r pn) :
    (accept r pn).lastPn = pn := by
  have h' : r.lastPn < pn := h
  simp [accept, h']

/-- The window never moves backwards, whatever arrives. -/
theorem accept_monotone (r : Rx) (pn : Nat) :
    r.lastPn ≤ (accept r pn).lastPn := by
  unfold accept
  split
  · next hlt => exact Nat.le_of_lt hlt
  · exact Nat.le_refl _

/-- A PN is delivered at most once: after processing it, the same PN is never
    fresh again. -/
theorem no_redelivery (r : Rx) (pn : Nat) : ¬ Fresh (accept r pn) pn := by
  unfold Fresh accept
  by_cases h : r.lastPn < pn
  · rw [if_pos h]; exact Nat.lt_irrefl pn
  · rw [if_neg h]; exact h

/-- Process a whole sequence of received PNs. -/
def acceptAll (r : Rx) : List Nat → Rx
  | [] => r
  | pn :: rest => acceptAll (accept r pn) rest

/-- The window is monotone over any received sequence. -/
theorem acceptAll_monotone (r : Rx) (pns : List Nat) :
    r.lastPn ≤ (acceptAll r pns).lastPn := by
  induction pns generalizing r with
  | nil => exact Nat.le_refl _
  | cons pn rest ih =>
    have h1 := accept_monotone r pn
    have h2 := ih (accept r pn)
    exact Nat.le_trans h1 h2

/-- Once a PN has been accepted it stays dead forever: however many frames
    follow, a replay of that PN is never fresh, so it is never delivered
    twice. -/
theorem accepted_pn_dead_forever (r : Rx) (pn : Nat) (h : Fresh r pn)
    (later : List Nat) : ¬ Fresh (acceptAll (accept r pn) later) pn := by
  have hadv := accept_advances r pn h
  have hmono := acceptAll_monotone (accept r pn) later
  simp only [Fresh]
  omega

end Nonos.CcmpReplay
