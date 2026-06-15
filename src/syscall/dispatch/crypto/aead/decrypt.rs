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

use super::{algorithm, constants::MAX_AEAD_PLAINTEXT, constants::TAG_LEN, copy};
use crate::capabilities::Capability;
use crate::syscall::dispatch::crypto::error::{map_capsule_error, CryptoErrorContext};
use crate::syscall::dispatch::{errno, require_capability};
use crate::syscall::SyscallResult;

pub fn handle_crypto_decrypt(
    algo: u64,
    key_ptr: u64,
    nonce_ptr: u64,
    ciphertext_ptr: u64,
    ciphertext_len: u64,
    plaintext_ptr: u64,
) -> SyscallResult {
    if let Err(e) = require_capability(Capability::Crypto) {
        return e;
    }
    if let Err(e) = algorithm::require_known(algo) {
        return e;
    }
    if ciphertext_len as usize <= TAG_LEN {
        return errno(22);
    }
    let key = match copy::read_array::<32>(key_ptr) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let nonce = match copy::read_array::<12>(nonce_ptr) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let max_ciphertext = MAX_AEAD_PLAINTEXT + TAG_LEN;
    let ciphertext = match copy::read_vec(ciphertext_ptr, ciphertext_len, max_ciphertext) {
        Ok(v) => v,
        Err(e) => return e,
    };
    match algorithm::open(algo, &key, &nonce, &[], &ciphertext) {
        Ok(pt) => copy::write_result(plaintext_ptr, &pt),
        Err(e) => map_capsule_error(e, CryptoErrorContext::Authenticated),
    }
}
