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

use super::constants::{PUBLIC_KEY_BYTES, SIGNATURE_BYTES};
use super::ffi::PQCLEAN_MLDSA65_CLEAN_crypto_sign_verify;

pub fn verify_signature(pubkey: &[u8], msg: &[u8], sig: &[u8]) -> bool {
    if pubkey.len() != PUBLIC_KEY_BYTES || sig.len() != SIGNATURE_BYTES {
        return false;
    }
    let rc = unsafe {
        PQCLEAN_MLDSA65_CLEAN_crypto_sign_verify(
            sig.as_ptr(),
            sig.len(),
            msg.as_ptr(),
            msg.len(),
            pubkey.as_ptr(),
        )
    };
    rc == 0
}
