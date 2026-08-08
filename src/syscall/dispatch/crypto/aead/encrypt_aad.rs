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

use super::{algorithm, constants::MAX_AEAD_PLAINTEXT, copy, frame};
use crate::capabilities::Capability;
use crate::syscall::dispatch::crypto::error::{map_capsule_error, CryptoErrorContext};
use crate::syscall::dispatch::require_capability;
use crate::syscall::SyscallResult;

pub fn handle_crypto_encrypt_aad(
    algo: u64,
    key_ptr: u64,
    nonce_ptr: u64,
    frame_ptr: u64,
    frame_len: u64,
    ciphertext_ptr: u64,
) -> SyscallResult {
    if let Err(e) = require_capability(Capability::Crypto) {
        return e;
    }
    if let Err(e) = algorithm::require_known(algo) {
        return e;
    }
    let key = match copy::read_array::<32>(key_ptr) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let nonce = match copy::read_array::<12>(nonce_ptr) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let raw = match copy::read_vec(frame_ptr, frame_len, MAX_AEAD_PLAINTEXT + 260) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let (aad, plaintext) = match frame::split(&raw) {
        Ok(v) => v,
        Err(e) => return e,
    };
    match algorithm::seal(algo, &key, &nonce, aad, plaintext) {
        Ok(ct) => copy::write_result(ciphertext_ptr, &ct),
        Err(e) => map_capsule_error(e, CryptoErrorContext::Authenticated),
    }
}
