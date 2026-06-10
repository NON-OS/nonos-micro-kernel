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

use super::state::KEYSTORE;
use super::types::{RevocationReason, PK_LEN};
use super::util::derive_keyid;
use crate::log::logger::{log_error, log_warn};

pub fn revoke_key_by_pubkey(
    pubkey: &[u8; PK_LEN],
    reason: RevocationReason,
    timestamp: u64,
) -> bool {
    let key_id = derive_keyid(pubkey);
    let mut store = KEYSTORE.lock();
    if store.revoke_key(key_id, reason, timestamp) {
        log_warn("crypto", "key revoked");
        true
    } else {
        log_error("crypto", "revocation failed");
        false
    }
}
