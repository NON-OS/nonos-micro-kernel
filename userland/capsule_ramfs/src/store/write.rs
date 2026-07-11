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

use super::crypto::{fresh_nonce, open, seal, TAG_LEN};
use super::types::{Store, StoreError};

impl Store {
    pub fn write_at(
        &mut self,
        path: &str,
        offset: usize,
        data: &[u8],
    ) -> Result<usize, StoreError> {
        self.ensure(path)?;
        let f = self.files.get_mut(path).ok_or(StoreError::NotFound)?;
        let mut plain: Vec<u8> = if f.ciphertext.is_empty() {
            Vec::new()
        } else {
            let plain_len =
                f.ciphertext.len().checked_sub(TAG_LEN).ok_or(StoreError::CryptoFailure)?;
            let mut buf = vec![0u8; plain_len];
            let n = open(&f.key, &f.nonce, &f.ciphertext, &mut buf)?;
            buf.truncate(n);
            buf
        };
        let end = offset.saturating_add(data.len());
        if end > plain.len() {
            plain.resize(end, 0);
        }
        plain[offset..end].copy_from_slice(data);
        f.nonce = fresh_nonce()?;
        let mut cipher = vec![0u8; plain.len() + TAG_LEN];
        let n = seal(&f.key, &f.nonce, &plain, &mut cipher)?;
        cipher.truncate(n);
        f.ciphertext = cipher;
        Ok(data.len())
    }
}
