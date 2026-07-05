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

pub mod constant_time;
pub mod key_management;
pub mod random;
pub mod trusted_hashes;
pub mod trusted_keys;

pub use key_management::{
    delete_all_keys, delete_key, derive_key, export_key, find_key_by_fingerprint, generate_key,
    get_key_info, import_key, init as key_management_init, list_keys, list_keys_by_owner,
    rotate_key, use_key, KeyAuditEntry, KeyEntry, KeyError, KeyInfo, KeyOperation, KeyResult,
    KeyStore, KeyType, KeyUsage,
};

pub use constant_time::{
    aes_ct, ct_compare, ct_copy_bounded, ct_eq_u32, ct_eq_u64, ct_gt_u32, ct_hmac_verify,
    ct_lt_u32, ct_lt_u64, ct_max_u32, ct_min_u32, ct_select_slice, ct_select_u32, ct_select_u64,
    ct_select_u8, ct_signature_verify, ct_swap_slices, ct_verify, ct_zero, ct_zero_u64, ed25519_ct,
    init as constant_time_init, x25519_ct, CtVerifyResult, SelfCheckResult, TimingMode,
};

pub use random::{
    fill_random, fill_random_bytes, init as random_init, secure_random_u32, secure_random_u64,
    secure_random_u8, try_fill_random,
};

pub use trusted_keys::{
    add_trusted_key, get_trusted_key, get_trusted_keys, init as trusted_keys_init,
    init_trusted_keys, list_trusted_keys, verify_signature, TrustedKey, TrustedKeyDB,
};

pub use trusted_hashes::{
    add_trusted_hash, get_trusted_hash, init as trusted_hashes_init, list_trusted_hashes,
    verify_integrity, TrustedHashDB,
};
