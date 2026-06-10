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

mod api_add;
mod api_init;
mod api_query;
mod api_revoke;
mod api_state;
mod api_validate;
mod store_add;
mod store_core;
mod store_revoke;
mod store_validate;
mod types;
mod util;

pub use api_add::{add_key, add_key_versioned};
pub use api_init::{init_nonos_keys, init_production_keys};
pub use api_query::{
    get_build_timestamp, get_key_fingerprint, get_minimum_version, get_nonos_key, get_nonos_key_id,
    is_initialized, key_count,
};
pub use api_revoke::{revoke_key_by_pubkey, set_minimum_version};
pub use api_state::{KEYSTORE, NONOS_SIGNING_KEY};
pub use api_validate::validate_key;
pub use store_core::KeyStore;
pub use types::{
    KeyId, KeyStatus, RevocationEntry, RevocationReason, MAX_KEYS, MAX_REVOKED, PK_LEN,
};
pub use util::derive_keyid;
