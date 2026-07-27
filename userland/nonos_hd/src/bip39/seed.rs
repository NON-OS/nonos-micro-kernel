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

use crate::pbkdf2::pbkdf2_hmac_sha512;
use crate::wipe::wipe;
use crate::wordlist::ENGLISH_WORDLIST;

use super::MAX_WORDS;

// The longest wordlist entry is 8 letters; 24 words with separating spaces
// bound the phrase, and "mnemonic" plus a passphrase bound the salt.
const PHRASE_MAX: usize = MAX_WORDS * 9;
const PASSPHRASE_MAX: usize = 128;
const SALT_MAX: usize = 8 + PASSPHRASE_MAX;

/// Derive the 64-byte BIP39 seed: PBKDF2-HMAC-SHA512 with 2048 rounds over
/// the space-joined phrase, salted with "mnemonic" plus the passphrase. The
/// phrase is assembled in a fixed buffer and wiped before returning. Returns
/// false (and a zeroed `out`) for an invalid index or an oversized
/// passphrase, never a seed derived from something other than the words.
pub fn seed_from_words(indices: &[u16], passphrase: &[u8], out: &mut [u8; 64]) -> bool {
    if indices.is_empty() || indices.len() > MAX_WORDS || passphrase.len() > PASSPHRASE_MAX {
        wipe(out);
        return false;
    }

    let mut phrase = [0u8; PHRASE_MAX];
    let mut len = 0usize;
    for (i, &index) in indices.iter().enumerate() {
        let Some(word) = ENGLISH_WORDLIST.get(index as usize) else {
            wipe(&mut phrase);
            wipe(out);
            return false;
        };
        if i > 0 {
            phrase[len] = b' ';
            len += 1;
        }
        let bytes = word.as_bytes();
        phrase[len..len + bytes.len()].copy_from_slice(bytes);
        len += bytes.len();
    }

    let mut salt = [0u8; SALT_MAX];
    salt[..8].copy_from_slice(b"mnemonic");
    salt[8..8 + passphrase.len()].copy_from_slice(passphrase);

    pbkdf2_hmac_sha512(&phrase[..len], &salt[..8 + passphrase.len()], 2048, out);

    wipe(&mut phrase);
    wipe(&mut salt);
    true
}
