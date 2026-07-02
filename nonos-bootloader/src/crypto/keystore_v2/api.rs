// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::store_core::KeystoreV2;
use super::types_key::KeyType;
use super::types_trusted_key::TrustedKey;
use spin::Mutex;

pub static KEYSTORE_V2: Mutex<KeystoreV2> = Mutex::new(KeystoreV2::new());

include!(concat!(env!("OUT_DIR"), "/keys_generated.rs"));

pub fn init_production_keystore() -> Result<usize, &'static str> {
    let mut store = KEYSTORE_V2.lock();
    if NONOS_MLDSA65_PUBLIC_KEY.iter().all(|&b| b == 0) {
        return Err("ML-DSA-65 key missing");
    }
    let primary_key =
        TrustedKey::new(NONOS_PUBLIC_KEY, KEY_VERSION, BUILD_TIMESTAMP, 0, KeyType::Primary);
    if primary_key.key_id != NONOS_KEY_ID {
        return Err("key ID mismatch");
    }
    store.add_key(primary_key)?;
    Ok(store.key_count)
}

pub fn get_keystore_fingerprint() -> &'static str {
    KEY_FINGERPRINT
}

pub fn wipe_all_keys() {
    let mut store = KEYSTORE_V2.lock();
    for key in store.keys.iter_mut() {
        key.zeroize();
    }
    store.key_count = 0;
    for rev in store.revocations.iter_mut() {
        crate::security::memory::zeroize_32(rev);
    }
    store.revocation_count = 0;
}
