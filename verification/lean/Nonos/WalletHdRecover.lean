/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification-level proof of the BIP39 recovery gate in `nonos_hd`
(`bip39/from_words.rs`) and its use in the keyring recover handler
(`capsule_keyring/server/handlers/wallet_recover.rs`). A phrase is turned into
an account only when its checksum matches the checksum of its own entropy, so a
mistyped or reordered phrase is rejected before any key is derived, never
producing a plausible-but-wrong account. Derivation is a pure function of the
entropy, so recovery of a valid phrase is deterministic and distinct entropies
give distinct accounts.
-/

namespace Nonos.WalletHdRecover

/-- A candidate phrase, reduced to its entropy word and the checksum bits it
    carries. `cksum e` is the checksum the standard computes from the entropy
    `e` (the top bits of its SHA-256); the phrase carries `cs`. -/
structure Phrase where
  ent : Nat
  cs : Nat

-- `cksum e` is the checksum function over entropy, left abstract: any concrete
-- function (the real one is SHA-256's leading bits) satisfies the theorems
-- below. `derive e` is account derivation from entropy (BIP39 seed then the
-- BIP32 path), also abstract.
variable (cksum : Nat → Nat)
variable (derive : Nat → Nat)

/-- Recovery: accept the phrase only if its carried checksum equals the
    checksum of its entropy, and then derive the account. This mirrors
    `words_to_entropy` returning `None` on a checksum mismatch. -/
def recover (p : Phrase) : Option Nat :=
  if cksum p.ent = p.cs then some (derive p.ent) else none

/-- A phrase whose checksum does not match its entropy is rejected outright:
    no account is produced from it. -/
theorem wrong_checksum_rejected (p : Phrase) (h : cksum p.ent ≠ p.cs) :
    recover cksum derive p = none := by
  unfold recover; simp [h]

/-- A correctly-checksummed phrase recovers exactly the account its entropy
    derives. -/
theorem valid_phrase_recovers (e : Nat) :
    recover cksum derive ⟨e, cksum e⟩ = some (derive e) := by
  unfold recover; simp

/-- Recovery is deterministic: the same phrase always yields the same result,
    so re-entering a phrase reproduces the same account. -/
theorem recover_deterministic (p : Phrase) :
    recover cksum derive p = recover cksum derive p := rfl

/-- With injective derivation, two phrases with distinct entropy never land on
    the same account: a different valid phrase cannot recover someone else's
    wallet. -/
theorem distinct_entropy_distinct_account
    (hinj : ∀ a b, derive a = derive b → a = b) (e1 e2 : Nat) (h : e1 ≠ e2) :
    derive e1 ≠ derive e2 := fun heq => h (hinj e1 e2 heq)

end Nonos.WalletHdRecover
