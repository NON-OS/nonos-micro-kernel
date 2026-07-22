/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification-level proof of the encrypted-at-rest seed protocol: the seed is
sealed under an AEAD keyed by a TPM-sealed key and written to persistent
storage, then unsealed after a reboot to reproduce the exact same account.
The AEAD's own correctness (decrypt inverts encrypt) and authenticity (a tag
made under one key does not verify under another) are the crate's obligations,
proven by the ChaCha20-Poly1305 round-trip host test; taken as hypotheses here,
they give the two protocol guarantees: a reboot with the right key recovers the
seed, and ciphertext read with any other key is rejected rather than decrypted
to garbage.
-/

namespace Nonos.WalletSeedSeal

/-- An AEAD instance over abstract key, nonce, plaintext and tag types.
    `enc`/`dec` are the cipher, `mac` the authentication tag. -/
structure Aead (K N P C T : Type) where
  enc : K → N → P → C
  dec : K → N → C → P
  mac : K → N → C → T

/-- Seal a plaintext: ciphertext plus its authentication tag. -/
def mkSeal {K N P C T} (a : Aead K N P C T) (k : K) (n : N) (p : P) : C × T :=
  (a.enc k n p, a.mac k n (a.enc k n p))

/-- Open a sealed blob: recompute the tag under the presented key and decrypt
    only if it matches the stored tag. -/
def openSeal {K N P C T} [DecidableEq T]
    (a : Aead K N P C T) (k : K) (n : N) (ct : C × T) : Option P :=
  if a.mac k n ct.1 = ct.2 then some (a.dec k n ct.1) else none

/-- Reboot recovery: sealing with a key and opening the persisted ciphertext
    with the same key reproduces the exact seed. Needs only cipher correctness
    (`dec` inverts `enc`), which the round-trip test establishes. -/
theorem reboot_preserves {K N P C T} [DecidableEq T]
    (a : Aead K N P C T) (k : K) (n : N) (p : P)
    (hcorrect : ∀ q, a.dec k n (a.enc k n q) = q) :
    openSeal a k n (mkSeal a k n p) = some p := by
  unfold openSeal mkSeal
  simp [hcorrect]

/-- Off-machine / wrong-key rejection: ciphertext sealed under `k` and read
    with a different key `k'` does not decrypt. Needs authenticity: a tag made
    under `k` does not verify under `k'`. So the persisted blob is worthless
    without the TPM-sealed key. -/
theorem wrong_key_rejected {K N P C T} [DecidableEq T]
    (a : Aead K N P C T) (k k' : K) (n : N) (p : P)
    (hauth : a.mac k' n (a.enc k n p) ≠ a.mac k n (a.enc k n p)) :
    openSeal a k' n (mkSeal a k n p) = none := by
  unfold openSeal mkSeal
  simp [hauth]

/-- Sealing the same seed twice under distinct nonces still recovers it under
    each: nonce discipline (never reuse) does not cost recoverability. -/
theorem distinct_nonces_recover {K N P C T} [DecidableEq T]
    (a : Aead K N P C T) (k : K) (n1 n2 : N) (p : P)
    (hc1 : ∀ q, a.dec k n1 (a.enc k n1 q) = q)
    (hc2 : ∀ q, a.dec k n2 (a.enc k n2 q) = q) :
    openSeal a k n1 (mkSeal a k n1 p) = some p ∧
      openSeal a k n2 (mkSeal a k n2 p) = some p :=
  ⟨reboot_preserves a k n1 p hc1, reboot_preserves a k n2 p hc2⟩

end Nonos.WalletSeedSeal
