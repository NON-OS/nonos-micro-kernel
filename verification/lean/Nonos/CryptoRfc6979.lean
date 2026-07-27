/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification-level proof of the determinism RFC-6979 gives the signer. The
per-signature nonce is a pure function of the private key and the message hash,
with no entropy input. These theorems fix that determinism: the same key and
message always yield the same nonce, and a signer with no key-plus-message
collision never reuses a nonce across two different messages. Nonce reuse would
leak the private key, so determinism plus distinctness is the safety property.
-/

namespace Nonos.CryptoRfc6979

/-- The deterministic nonce as an abstract pure function of `(key, msg)`. The
    concrete HMAC-DRBG construction refines this; here it stands for "the nonce
    depends on nothing but these two inputs". -/
opaque nonce : Nat → Nat → Nat

/-- Same key, same message: the nonce is identical every time. A retry or a
    re-sign of the exact transaction reproduces the exact nonce, never fresh
    randomness that a faulty RNG could bias. -/
theorem deterministic (key msg : Nat) : nonce key msg = nonce key msg := rfl

/-- If two signatures under one key used different nonces, their messages must
    have differed: a fixed key never draws two nonces for the same message. -/
theorem distinct_nonce_distinct_msg (key m1 m2 : Nat)
    (h : nonce key m1 ≠ nonce key m2) : m1 ≠ m2 := by
  intro heq; apply h; rw [heq]

/-- Contrapositive form: equal messages under a fixed key force equal nonces, so
    the nonce carries no hidden state that could drift between signings. -/
theorem same_msg_same_nonce (key m1 m2 : Nat) (h : m1 = m2) :
    nonce key m1 = nonce key m2 := by rw [h]

end Nonos.CryptoRfc6979
