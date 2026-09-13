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

//! Redoing the machine's fold.
//!
//! The TPM signed thirty-two bytes and nothing else. Everything a receipt says
//! about what was running is carried by the claim that those bytes are the
//! digest of this set, and folding it here is the whole check.

use crate::hexcode::hex;
use crate::kernel::{DOMAIN, ENTRY_LEN};

/// What the machine would have signed, given these entries.
///
/// The count comes from the byte length, not from the sender.
pub fn fold_root(entries: &[u8]) -> [u8; 32] {
    let count = (entries.len() / ENTRY_LEN) as u32;
    let mut hasher = blake3::Hasher::new();
    hasher.update(DOMAIN);
    hasher.update(&count.to_be_bytes());
    hasher.update(entries);
    *hasher.finalize().as_bytes()
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RootMismatch {
    pub signed: [u8; 32],
    pub computed: [u8; 32],
}

impl RootMismatch {
    /// Says nothing about which entry differs. The reader has no authenticated
    /// set to compare against, so any such claim would be a guess.
    pub fn describe(&self) -> String {
        format!(
            "the entries fold to {} and the signature is over {}: \
             these entries are not the set this machine signed",
            hex(&self.computed),
            hex(&self.signed)
        )
    }
}

/// Checks the entries against the root the document's signature covers.
pub fn verify_root(entries: &[u8], signed: [u8; 32]) -> Result<(), RootMismatch> {
    let computed = fold_root(entries);
    /*
     * Both values are public and the reader holds no secret, so a variable
     * time compare leaks nothing.
     */
    if computed == signed {
        Ok(())
    } else {
        Err(RootMismatch { signed, computed })
    }
}
