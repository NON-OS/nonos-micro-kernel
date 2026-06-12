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
use super::display::{print, print_hex_bytes};
use super::types::CryptoVerifyResult;
use super::verify_error::display_verification_error;
use crate::crypto::sig::verify_signature_bytes;
use crate::log::logger::{log_error, log_info};

pub fn verify_and_display_signature(
    kernel_code: &[u8],
    signature: &[u8],
    result: &mut CryptoVerifyResult,
    st: &mut SystemTable<Boot>,
) {
    print(st, cstr16!("  [CRYPTO] Extracting Ed25519 signature...\r\n"));
    mini_delay();
    if signature.iter().all(|&b| b == 0) {
        log_error("crypto_real", "Signature is all zeros - kernel unsigned");
        print(st, cstr16!("  [CRYPTO] Signature: ALL ZEROS (UNSIGNED!)\r\n"));
        print(st, cstr16!("  [CRYPTO] Ed25519 verify ....................... [FAIL]\r\n"));
        return;
    }
    display_signature_components(signature, st);
    print(st, cstr16!("  [CRYPTO] Verifying Ed25519 signature...\r\n"));
    match verify_signature_bytes(kernel_code, signature) {
        Ok(key_id) => signature_passed(result, &key_id, st),
        Err(e) => {
            result.signature_valid = false;
            log_error("kernel_verify", "Ed25519 signature verification FAILED");
            print(st, cstr16!("  [CRYPTO] Ed25519 verify ....................... [FAIL]\r\n"));
            display_verification_error(e, st);
        }
    }
    mini_delay();
}

fn signature_passed(result: &mut CryptoVerifyResult, key_id: &[u8], st: &mut SystemTable<Boot>) {
    result.signature_valid = true;
    log_info("kernel_verify", "Ed25519 signature VERIFIED against trusted key");
    print(st, cstr16!("  [CRYPTO] Ed25519 verify ....................... [PASS]\r\n"));
    mini_delay();
    print(st, cstr16!("  [CRYPTO] Signer key ID: "));
    print_hex_bytes(st, &key_id[0..8]);
    print(st, cstr16!("...\r\n"));
}

fn display_signature_components(signature: &[u8], st: &mut SystemTable<Boot>) {
    print(st, cstr16!("  [CRYPTO] Sig R: "));
    print_hex_bytes(st, &signature[0..8]);
    print(st, cstr16!("...\r\n"));
    print(st, cstr16!("  [CRYPTO] Sig S: "));
    print_hex_bytes(st, &signature[32..40]);
    print(st, cstr16!("...\r\n"));
    mini_delay();
}
