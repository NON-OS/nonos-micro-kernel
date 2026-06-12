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

use super::state::{CURRENT_VERSION, INIT_DONE, KEYSTORE};
use super::types::{KeyId, PK_LEN};
use core::sync::atomic::Ordering;

pub fn add_key_versioned(pubkey: &[u8; PK_LEN], version: u32) -> Result<KeyId, &'static str> {
    let mut store = KEYSTORE.lock();
    let result = store.add_key(pubkey, version);
    if result.is_ok() {
        INIT_DONE.store(true, Ordering::SeqCst);
    }
    result
}

pub fn add_key(pubkey: &[u8; PK_LEN]) -> Result<KeyId, &'static str> {
    add_key_versioned(pubkey, CURRENT_VERSION.load(Ordering::SeqCst))
}
