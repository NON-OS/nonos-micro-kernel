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

use super::state::{BUILD_TIMESTAMP, KEYSTORE, KEY_FINGERPRINT, NONOS_KEY_ID, NONOS_PUBLIC_KEY};
use super::types::{KeyStatus, PK_LEN};

pub fn key_count() -> usize {
    KEYSTORE.lock().count
}
pub fn get_minimum_version() -> u32 {
    KEYSTORE.lock().minimum_version
}
pub fn get_nonos_key() -> &'static [u8; 32] {
    &NONOS_PUBLIC_KEY
}
pub fn get_nonos_key_id() -> &'static [u8; 32] {
    &NONOS_KEY_ID
}
pub fn get_key_fingerprint() -> &'static str {
    KEY_FINGERPRINT
}
pub fn get_build_timestamp() -> u64 {
    BUILD_TIMESTAMP
}

pub fn validate_key(pubkey: &[u8; PK_LEN], version: u32) -> KeyStatus {
    KEYSTORE.lock().validate_key(pubkey, version)
}

pub fn set_minimum_version(version: u32) -> bool {
    let mut store = KEYSTORE.lock();
    if version > store.minimum_version {
        store.minimum_version = version;
        crate::log::logger::log_info("crypto", "minimum version updated");
        true
    } else {
        false
    }
}
