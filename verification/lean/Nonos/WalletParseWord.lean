/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification-level proof for the wallet's ABI word extractor
(`rpc/parse_call_word.rs`). A tuple return packs each field as a 32-byte word;
word `i` lives at hex offsets `[i*64, i*64+64)`. These theorems fix that the
extractor only reads a word the response is long enough to hold, and that
distinct words never overlap, so a stats field is never read from another's
bytes.
-/

namespace Nonos.WalletParseWord

/-- First hex-character offset of word `i`. -/
def start (i : Nat) : Nat := i * 64
/-- One past the last hex-character offset of word `i`. -/
def stop (i : Nat) : Nat := i * 64 + 64

/-- The extractor accepts word `i` exactly when the response holds its 64 hex
    characters. Mirrors the `hex.len() < end` guard. -/
def readable (i len : Nat) : Prop := stop i ≤ len

/-- Each word is exactly 64 hex characters (32 bytes) wide. -/
theorem word_width (i : Nat) : stop i - start i = 64 := by
  unfold start stop; omega

/-- Word `i` and word `j` never overlap when `i ≠ j`: one ends at or before the
    other begins, so a field is read only from its own bytes. -/
theorem words_disjoint (i j : Nat) (h : i < j) : stop i ≤ start j := by
  unfold start stop; omega

/-- Readability is monotone downward: if word `i` fits, so does every earlier
    word, so reading word 0 of a valid multi-word return never runs short. -/
theorem earlier_words_readable (i j len : Nat) (h : j ≤ i) (hr : readable i len) :
    readable j len := by
  unfold readable stop at *; omega

end Nonos.WalletParseWord
