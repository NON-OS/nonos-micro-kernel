/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Custody of a signing seed at rest. A seed on disk is the identity every
signature below it depends on, so what an attacker gets from reading the file
is the whole question. The theorems below show that a plain seed hands over the
key to anyone who can read it, that a sealed one hands over nothing without the
passphrase, that the cost parameters cannot be edited to weaken the derivation
without the seed refusing to open, and that the two forms can never be mistaken
for one another.

The authenticated cipher is a parameter carrying its own correctness and
authenticity, not an assumed axiom: every theorem here holds for any cipher
meeting the stated interface, and the corpus keeps its property of assuming
nothing.
-/

namespace Nonos.SeedCustody

/-- An authenticated cipher, carrying the two properties the argument needs.

`unlock` returns the message only for the key it was sealed under, and only for
a ciphertext that key actually produced. Those are the guarantees an AEAD tag
is for, stated as fields so nothing is taken on faith. -/
structure Aead (Key Msg Cipher : Type) where
  lock : Key → Msg → Cipher
  unlock : Key → Cipher → Option Msg
  /-- The right key recovers exactly what was sealed. -/
  correct : ∀ k m, unlock k (lock k m) = some m
  /-- Anything that opens under a key was sealed under that key. -/
  authentic : ∀ k c m, unlock k c = some m → lock k m = c
  /-- A ciphertext commits to the key that produced it.

  This is key commitment, and it is the reason the cost parameters in the
  header cannot be edited: they feed the derivation, so a changed header is a
  changed key, and a changed key cannot be the one this body committed to. It
  is stated because a general AEAD does not provide it: ChaCha20-Poly1305 needs
  a committing construction for this argument to transfer, and pretending
  otherwise would prove something about a cipher nobody has. -/
  committing : ∀ k k' m, lock k m = lock k' m → k = k'

/-- How a seed sits on disk. `plain` is the shape that shipped before custody;
`sealed` carries the derivation cost, the salt, and the ciphertext. -/
inductive Stored (Cipher : Type) where
  | plain (seed : Nat)
  | locked (mKib tCost pCost salt : Nat) (body : Cipher)

/-- What a reader of the file learns without knowing the passphrase.

A plain seed yields its bytes to anyone who can read it. A sealed one yields
nothing, which is the entire point of the format. -/
def readsWithoutSecret {Cipher : Type} : Stored Cipher → Option Nat
  | .plain s => some s
  | .locked _ _ _ _ _ => none

/-- Reading a plain seed hands over the key. -/
theorem plain_reveals {Cipher : Type} (s : Nat) :
    readsWithoutSecret (Cipher := Cipher) (.plain s) = some s := rfl

/-- Reading a sealed seed hands over nothing. -/
theorem sealed_reveals_nothing {Cipher : Type}
    (m t p salt : Nat) (body : Cipher) :
    readsWithoutSecret (.locked m t p salt body) = none := rfl

/-- The two forms are never confused: nothing is both plain and sealed. This is
what keeps a sealed file from being parsed as a key. -/
theorem plain_ne_sealed {Cipher : Type}
    (s m t p salt : Nat) (body : Cipher) :
    (Stored.plain s : Stored Cipher) ≠ .locked m t p salt body := by
  intro h
  cases h

/-- Key derivation. The passphrase alone does not fix the key: the salt and the
three cost parameters go in with it, which is what binds the header to the
body. -/
structure Kdf (Phrase Key : Type) where
  derive : Phrase → (salt mKib tCost pCost : Nat) → Key
  /-- Different derivation inputs give a different key. Stated as injectivity,
  which is the property the binding argument below actually needs. -/
  injective : ∀ ph ph' s s' m m' t t' p p',
    derive ph s m t p = derive ph' s' m' t' p' →
    ph = ph' ∧ s = s' ∧ m = m' ∧ t = t' ∧ p = p'

/-- Opening a stored seed: derive the key from the passphrase and the header,
then let the cipher decide. -/
def recover {Phrase Key Cipher : Type}
    (kdf : Kdf Phrase Key) (aead : Aead Key Nat Cipher)
    (ph : Phrase) : Stored Cipher → Option Nat
  | .plain s => some s
  | .locked m t p salt body => aead.unlock (kdf.derive ph salt m t p) body

/-- Sealing a seed under a passphrase and a chosen cost. -/
def protectSeed {Phrase Key Cipher : Type}
    (kdf : Kdf Phrase Key) (aead : Aead Key Nat Cipher)
    (ph : Phrase) (salt m t p seed : Nat) : Stored Cipher :=
  .locked m t p salt (aead.lock (kdf.derive ph salt m t p) seed)

/-- The passphrase that sealed a seed opens it, and returns exactly that seed.
Custody is worth nothing if the owner cannot get their own key back. -/
theorem recover_seal {Phrase Key Cipher : Type}
    (kdf : Kdf Phrase Key) (aead : Aead Key Nat Cipher)
    (ph : Phrase) (salt m t p seed : Nat) :
    recover kdf aead ph (protectSeed kdf aead ph salt m t p seed) = some seed := by
  simp [recover, protectSeed, aead.correct]

/-- Editing the recorded cost to a weaker one stops the seed opening.

An attacker who could lower the work factor in the header and still open the
file would have made the derivation cheap to brute force. The header is not
covered by the tag directly; it is bound because the key derives from it, and
that is what this states. -/
theorem weakened_cost_does_not_open {Phrase Key Cipher : Type}
    (kdf : Kdf Phrase Key) (aead : Aead Key Nat Cipher)
    (ph : Phrase) (salt m t p m' seed : Nat) (hm : m ≠ m') :
    recover kdf aead ph (.locked m' t p salt
      (aead.lock (kdf.derive ph salt m t p) seed)) ≠ some seed := by
  intro h
  have hopen : aead.unlock (kdf.derive ph salt m' t p)
      (aead.lock (kdf.derive ph salt m t p) seed) = some seed := h
  have hseal := aead.authentic _ _ _ hopen
  have hkey := aead.committing _ _ _ hseal
  have := kdf.injective ph ph salt salt m' m t t p p hkey
  exact hm (this.2.2.1).symm

/-- A seed sealed under one passphrase does not open under another, unless the
two derive the same key, which the derivation's injectivity rules out. -/
theorem wrong_passphrase_does_not_open {Phrase Key Cipher : Type}
    (kdf : Kdf Phrase Key) (aead : Aead Key Nat Cipher)
    (ph ph' : Phrase) (salt m t p seed : Nat) (hne : ph ≠ ph') :
    recover kdf aead ph' (protectSeed kdf aead ph salt m t p seed) ≠ some seed := by
  intro h
  have hopen : aead.unlock (kdf.derive ph' salt m t p)
      (aead.lock (kdf.derive ph salt m t p) seed) = some seed := h
  have hseal := aead.authentic _ _ _ hopen
  have hkey := aead.committing _ _ _ hseal
  have := kdf.injective ph' ph salt salt m m t t p p hkey
  exact hne (this.1).symm

/-- Sealing never produces the plain form, so migrating a seed always moves it
out of the shape that reveals the key. -/
theorem sealing_leaves_plain {Phrase Key Cipher : Type}
    (kdf : Kdf Phrase Key) (aead : Aead Key Nat Cipher)
    (ph : Phrase) (salt m t p seed s : Nat) :
    protectSeed kdf aead ph salt m t p seed ≠ .plain s := by
  intro h
  cases h

/-- After sealing, a reader without the passphrase learns nothing, whatever the
seed was. This is the custody property the format exists to provide. -/
theorem sealed_seed_is_opaque {Phrase Key Cipher : Type}
    (kdf : Kdf Phrase Key) (aead : Aead Key Nat Cipher)
    (ph : Phrase) (salt m t p seed : Nat) :
    readsWithoutSecret (protectSeed kdf aead ph salt m t p seed) = none := rfl

end Nonos.SeedCustody
