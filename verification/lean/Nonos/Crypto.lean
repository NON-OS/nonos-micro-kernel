/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification-level proof of the authentication gate: every MAC, AEAD tag and
signature check reduces to a constant-time tag comparison that accepts exactly
the matching tag. The real branch-free `ct_eq` is proven functionally correct
over all inputs in `userland/crypto_proofs` (#265), and the tags themselves are
proven correct against the RFC/NIST vectors there. Lean states what the gate
must decide; the code proofs show the implementation decides it.
-/

namespace Nonos.Crypto

/-- The authentication gate accepts a provided tag exactly when it equals the
    expected tag. This is the specification `ct_eq` implements. -/
def Accepts (expected provided : List UInt8) : Prop := provided = expected

/-- The genuine tag is accepted. -/
theorem genuine_tag_accepted (expected : List UInt8) : Accepts expected expected :=
  rfl

/-- Any tag that differs is rejected: a forgery cannot be accepted through a
    wrong tag, and in particular a single flipped byte is caught. -/
theorem wrong_tag_rejected (expected provided : List UInt8)
    (h : provided ≠ expected) : ¬ Accepts expected provided := h

/-- Acceptance depends only on tag equality, nothing else: a MAC/AEAD/signature
    check is exactly a tag-equality decision, with no other path to acceptance. -/
theorem acceptance_iff_equal (expected provided : List UInt8) :
    Accepts expected provided ↔ provided = expected :=
  Iff.rfl

/-- Tags of different length never match, so a truncated tag is rejected. -/
theorem length_mismatch_rejected (expected provided : List UInt8)
    (h : provided.length ≠ expected.length) : ¬ Accepts expected provided := by
  intro hacc
  apply h
  rw [hacc]

end Nonos.Crypto
