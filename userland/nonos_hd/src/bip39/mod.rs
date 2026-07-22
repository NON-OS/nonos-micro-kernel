// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! BIP39: entropy to mnemonic words, words back to entropy with checksum
//! verification, and phrase to seed via PBKDF2-HMAC-SHA512 with 2048 rounds.
//! Mnemonics are handled as u16 wordlist indices, not strings, so the phrase
//! only ever materializes inside `seed_from_words` in a fixed buffer that is
//! wiped before it returns.

mod from_words;
mod seed;
mod to_words;
mod word_index;

pub use from_words::words_to_entropy;
pub use seed::seed_from_words;
pub use to_words::entropy_to_words;
pub use word_index::word_index;

/// Longest supported mnemonic (24 words, 256-bit entropy).
pub const MAX_WORDS: usize = 24;
