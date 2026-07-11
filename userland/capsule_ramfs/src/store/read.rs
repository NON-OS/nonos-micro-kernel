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

use alloc::vec;
use alloc::vec::Vec;

use super::crypto::{open, TAG_LEN};
use super::types::{Store, StoreError};

impl Store {
    pub fn read_at(&self, path: &str, offset: usize, count: usize) -> Result<Vec<u8>, StoreError> {
        let f = self.files.get(path).ok_or(StoreError::NotFound)?;
        if f.ciphertext.is_empty() {
            return Ok(Vec::new());
        }
        // Non-empty ciphertext must carry at least the AEAD tag; checked_sub
        // avoids a usize underflow (and a huge allocation) on a truncated blob.
        let plain_len = f.ciphertext.len().checked_sub(TAG_LEN).ok_or(StoreError::CryptoFailure)?;
        let mut plain = vec![0u8; plain_len];
        let n = open(&f.key, &f.nonce, &f.ciphertext, &mut plain)?;
        plain.truncate(n);
        if offset >= plain.len() {
            return Ok(Vec::new());
        }
        let end = offset.saturating_add(count).min(plain.len());
        Ok(plain[offset..end].to_vec())
    }
}
