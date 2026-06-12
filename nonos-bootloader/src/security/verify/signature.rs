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

use super::constants::SIGNATURE_SIZE;
use crate::crypto::sig::{is_initialized as crypto_initialized, verify_signature_bytes};
use crate::log::logger::log_error;

pub fn verify_signature(data: &[u8], signature: &[u8]) -> bool {
    if !crypto_initialized() {
        log_error("security", "Cannot verify - crypto not initialized");
        return false;
    }
    if signature.len() != SIGNATURE_SIZE {
        log_error("security", "Invalid signature size");
        return false;
    }
    match verify_signature_bytes(data, signature) {
        Ok(_) => true,
        Err(_) => false,
    }
}
