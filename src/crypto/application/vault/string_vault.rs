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

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use spin::RwLock;

use crate::crypto::CryptoResult;

pub(super) static STRING_KEY_VAULT: RwLock<BTreeMap<String, Vec<u8>>> =
    RwLock::new(BTreeMap::new());

pub fn init_vault() -> CryptoResult<()> {
    STRING_KEY_VAULT.write().clear();
    Ok(())
}

pub fn store_key(key_id: &str, key: &[u8]) -> CryptoResult<()> {
    if key_id.is_empty() {
        return Err(crate::crypto::CryptoError::InvalidInput);
    }
    if key.is_empty() {
        return Err(crate::crypto::CryptoError::InvalidInput);
    }

    STRING_KEY_VAULT.write().insert(String::from(key_id), key.to_vec());
    Ok(())
}

pub fn retrieve_key(key_id: &str) -> CryptoResult<Vec<u8>> {
    if key_id.is_empty() {
        return Err(crate::crypto::CryptoError::InvalidInput);
    }

    STRING_KEY_VAULT.read().get(key_id).cloned().ok_or(crate::crypto::CryptoError::InvalidInput)
}

pub fn list_keys() -> CryptoResult<Vec<String>> {
    let vault = STRING_KEY_VAULT.read();
    Ok(vault.keys().cloned().collect())
}
