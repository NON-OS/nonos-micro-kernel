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

use uefi::cstr16;
use uefi::prelude::*;

use super::delay::mini_delay;
use super::display::print;
use super::signature_ed25519::verify_ed25519_signature;
use super::signature_hybrid::verify_hybrid_signature;
use super::signature_message::signed_kernel_message;
use super::types::CryptoVerifyResult;
use crate::image_format::types::SignatureAlgorithm;
use crate::log::logger::log_error;

pub fn verify_and_display_signature(
    kernel_hash: &[u8; 32],
    rollback_index: u32,
    algorithm: SignatureAlgorithm,
    signature: &[u8],
    result: &mut CryptoVerifyResult,
    st: &mut SystemTable<Boot>,
) {
    if signature.iter().all(|&b| b == 0) {
        reject_unsigned(result, st);
        return;
    }
    let message = signed_kernel_message(kernel_hash, rollback_index);
    match algorithm {
        SignatureAlgorithm::Ed25519 => {
            print(st, cstr16!("  [CRYPTO] Extracting Ed25519 signature...\r\n"));
            verify_ed25519_signature(&message, signature, result, st);
        }
        SignatureAlgorithm::Ed25519MlDsa65 => {
            print(st, cstr16!("  [CRYPTO] Extracting Ed25519 + ML-DSA-65 signatures...\r\n"));
            verify_hybrid_signature(&message, signature, result, st);
        }
    }
    mini_delay();
}

fn reject_unsigned(result: &mut CryptoVerifyResult, st: &mut SystemTable<Boot>) {
    result.signature_valid = false;
    log_error("crypto_real", "Signature is all zeros - kernel unsigned");
    print(st, cstr16!("  [CRYPTO] Signature: ALL ZEROS (UNSIGNED!)\r\n"));
    print(st, cstr16!("  [CRYPTO] Signature verify .................... [FAIL]\r\n"));
}
