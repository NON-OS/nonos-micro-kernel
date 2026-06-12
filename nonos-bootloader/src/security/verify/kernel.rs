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

use super::constants::{MIN_KERNEL_SIZE, SIGNATURE_SIZE};
use crate::crypto::sig::{
    is_initialized as crypto_initialized, verify_signature_bytes, VerifyError,
};
use crate::log::logger::{log_debug, log_error, log_info};

pub fn verify_kernel_signature_advanced(kernel_data: &[u8]) -> bool {
    if !crypto_initialized() {
        log_error("security", "CRITICAL: Cannot verify - crypto not initialized");
        return false;
    }
    if kernel_data.len() < MIN_KERNEL_SIZE {
        log_error("security", "Kernel too small for signature verification");
        return false;
    }
    let sig_offset = kernel_data.len().saturating_sub(SIGNATURE_SIZE);
    let kernel_code = &kernel_data[..sig_offset];
    let signature = &kernel_data[sig_offset..];
    if signature.len() != SIGNATURE_SIZE {
        log_error("security", "Invalid signature length in kernel");
        return false;
    }
    if signature.iter().all(|&b| b == 0) {
        log_error("security", "Signature is all zeros - rejected");
        return false;
    }
    let _hash = blake3::hash(kernel_code);
    log_debug("security", "Computed kernel hash for verification");
    match verify_signature_bytes(kernel_code, signature) {
        Ok(_key_id) => {
            log_info("security", "Kernel signature VERIFIED against trusted key");
            true
        }
        Err(VerifyError::InvalidSignature) => {
            log_error("security", "CRITICAL: Kernel signature INVALID - execution blocked");
            false
        }
        Err(VerifyError::NotInitialized) => {
            log_error("security", "CRITICAL: Signature verifier not initialized");
            false
        }
        Err(VerifyError::KeyNotFound) => {
            log_error("security", "CRITICAL: Signing key not in trusted store");
            false
        }
        Err(_) => {
            log_error("security", "Signature verification error");
            false
        }
    }
}
