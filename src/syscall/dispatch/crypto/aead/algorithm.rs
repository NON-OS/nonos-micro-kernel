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

use super::constants::{ALGO_AES256_GCM, ALGO_CHACHA20_POLY1305};
use crate::security::crypto_capsule::client as crypto_client;
use crate::security::crypto_capsule::CryptoCapsuleError;
use crate::syscall::dispatch::errno;
use crate::syscall::SyscallResult;
use alloc::vec::Vec;

pub(super) fn seal(
    algo: u64,
    key: &[u8; 32],
    nonce: &[u8; 12],
    aad: &[u8],
    plaintext: &[u8],
) -> Result<Vec<u8>, CryptoCapsuleError> {
    match algo {
        ALGO_CHACHA20_POLY1305 => crypto_client::chacha20_poly1305_seal(key, nonce, aad, plaintext),
        ALGO_AES256_GCM => crypto_client::aes256_gcm_seal(key, nonce, aad, plaintext),
        _ => Err(CryptoCapsuleError::InvalidArgument),
    }
}

pub(super) fn open(
    algo: u64,
    key: &[u8; 32],
    nonce: &[u8; 12],
    aad: &[u8],
    ciphertext: &[u8],
) -> Result<Vec<u8>, CryptoCapsuleError> {
    match algo {
        ALGO_CHACHA20_POLY1305 => {
            crypto_client::chacha20_poly1305_open(key, nonce, aad, ciphertext)
        }
        ALGO_AES256_GCM => crypto_client::aes256_gcm_open(key, nonce, aad, ciphertext),
        _ => Err(CryptoCapsuleError::InvalidArgument),
    }
}

pub(super) fn require_known(algo: u64) -> Result<(), SyscallResult> {
    match algo {
        ALGO_CHACHA20_POLY1305 | ALGO_AES256_GCM => Ok(()),
        _ => Err(errno(22)),
    }
}
