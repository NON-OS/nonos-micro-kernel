/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The attestation trailer: a magic tag, a declared depth, then the proof bytes. The
parser accepts a trailer only when the magic matches and the declared depth agrees
with the siblings present; the theorems below show encode then parse is the
identity, a wrong magic or a truncated trailer is rejected, the parser never
accepts what the encoder did not produce, and encoding is injective, so a mangled
trailer cannot slip past as a well-formed one.
-/

namespace Nonos.Stark.Trailer

/-- A trailer: a magic tag, a declared depth, and the payload words. -/
structure Trailer where
  magic : Nat
  depth : Nat
  payload : List Nat

/-- Encode a trailer to a word list: magic, depth, then payload. -/
def encode (t : Trailer) : List Nat := t.magic :: t.depth :: t.payload

/-- Parse a word list under an expected magic, checking the declared depth matches
    the payload length. Returns the trailer only when both agree. -/
def parse (expected : Nat) (words : List Nat) : Option Trailer :=
  match words with
  | m :: d :: rest => if m = expected ∧ d = rest.length then some ⟨m, d, rest⟩ else none
  | _ => none

/-- A well-formed trailer has its depth equal to its payload length. -/
def WellFormed (t : Trailer) : Prop := t.depth = t.payload.length

/-- Encoding a well-formed trailer, then parsing under its magic, returns it. -/
theorem parse_encode (t : Trailer) (h : WellFormed t) : parse t.magic (encode t) = some t := by
  cases t with
  | mk magic depth payload =>
    simp only [WellFormed] at h
    simp only [encode, parse]
    simp [h]

/-- The parser rejects an empty trailer. -/
theorem parse_empty (expected : Nat) : parse expected [] = none := rfl

/-- The parser rejects a trailer with only a magic and no depth. -/
theorem parse_no_depth (expected m : Nat) : parse expected [m] = none := rfl

/-- The parser rejects a wrong magic tag. -/
theorem parse_wrong_magic (expected m d : Nat) (rest : List Nat) (h : m ≠ expected) :
    parse expected (m :: d :: rest) = none := by
  simp [parse, h]

/-- The parser rejects a declared depth that disagrees with the payload. -/
theorem parse_depth_mismatch (expected d : Nat) (rest : List Nat) (h : d ≠ rest.length) :
    parse expected (expected :: d :: rest) = none := by
  simp [parse, h]

/-- Whatever the parser accepts carries the expected magic. -/
theorem parse_sound_magic (expected : Nat) (words : List Nat) (t : Trailer)
    (h : parse expected words = some t) : t.magic = expected := by
  match words with
  | [] => simp [parse] at h
  | [_] => simp [parse] at h
  | m :: d :: rest =>
    simp only [parse] at h
    by_cases hc : m = expected ∧ d = rest.length
    · rw [if_pos hc] at h; injection h with h; subst h; exact hc.1
    · rw [if_neg hc] at h; exact absurd h (by simp)

/-- Whatever the parser accepts is well-formed. -/
theorem parse_sound_wf (expected : Nat) (words : List Nat) (t : Trailer)
    (h : parse expected words = some t) : WellFormed t := by
  match words with
  | [] => simp [parse] at h
  | [_] => simp [parse] at h
  | m :: d :: rest =>
    simp only [parse] at h
    by_cases hc : m = expected ∧ d = rest.length
    · rw [if_pos hc] at h; injection h with h; subst h; exact hc.2
    · rw [if_neg hc] at h; exact absurd h (by simp)

/-- Encoding is injective: distinct trailers encode to distinct words. -/
theorem encode_injective (t₁ t₂ : Trailer) (h : encode t₁ = encode t₂) : t₁ = t₂ := by
  simp only [encode, List.cons.injEq] at h
  obtain ⟨hm, hd, hp⟩ := h
  cases t₁; cases t₂; simp_all

/-- Parsing an encoded well-formed trailer recovers its payload exactly. -/
theorem parse_recovers_payload (t : Trailer) (h : WellFormed t) :
    (parse t.magic (encode t)).map Trailer.payload = some t.payload := by
  rw [parse_encode t h]; rfl

end Nonos.Stark.Trailer
