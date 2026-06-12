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
use alloc::vec::Vec;
use spin::RwLock;

use super::types::KeyEntry;
use crate::crypto::CryptoResult;

pub(super) static KEY_VAULT: RwLock<BTreeMap<u32, KeyEntry>> = RwLock::new(BTreeMap::new());

pub fn get_signing_key(key_id: u32) -> Option<[u8; 32]> {
    let vault = KEY_VAULT.read();
    vault.get(&key_id).and_then(|entry| {
        if entry.private_key.len() >= 32 {
            let mut key = [0u8; 32];
            key.copy_from_slice(&entry.private_key[..32]);
            Some(key)
        } else {
            None
        }
    })
}

pub fn get_public_key(key_id: u32) -> Option<[u8; 32]> {
    let vault = KEY_VAULT.read();
    vault.get(&key_id).and_then(|entry| {
        if entry.public_key.len() >= 32 {
            let mut key = [0u8; 32];
            key.copy_from_slice(&entry.public_key[..32]);
            Some(key)
        } else {
            None
        }
    })
}

fn store_keypair(key_id: u32, private_key: &[u8], public_key: &[u8]) -> CryptoResult<()> {
    let entry = KeyEntry { private_key: private_key.to_vec(), public_key: public_key.to_vec() };
    KEY_VAULT.write().insert(key_id, entry);
    Ok(())
}

pub fn generate_and_store_ed25519_keypair() -> CryptoResult<u32> {
    let keypair = crate::crypto::ed25519::KeyPair::generate();

    static NEXT_KEY_ID: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(1);
    let key_id = NEXT_KEY_ID.fetch_add(1, core::sync::atomic::Ordering::SeqCst);

    store_keypair(key_id, &keypair.private, &keypair.public)?;

    Ok(key_id)
}

pub fn delete_vault_key(key_id: u32) -> CryptoResult<()> {
    let mut vault = KEY_VAULT.write();
    if let Some(mut entry) = vault.remove(&key_id) {
        for byte in entry.private_key.iter_mut() {
            unsafe { core::ptr::write_volatile(byte, 0) };
        }
        for byte in entry.public_key.iter_mut() {
            unsafe { core::ptr::write_volatile(byte, 0) };
        }
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn list_vault_keys() -> Vec<u32> {
    KEY_VAULT.read().keys().copied().collect()
}
