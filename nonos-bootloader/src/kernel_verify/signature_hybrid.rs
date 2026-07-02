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

use super::display::print;
use super::signature_ed25519::verify_ed25519_signature;
use super::types::CryptoVerifyResult;
use crate::crypto::mldsa65;
use crate::log::logger::{log_error, log_info};
use super::signature_policy::{policy_identity_ok, ED_SIG_START, PQ_ID_START, PQ_SIG_START};

pub fn verify_hybrid_signature(
    message: &[u8],
    signature: &[u8],
    result: &mut CryptoVerifyResult,
    st: &mut SystemTable<Boot>,
) {
    if !policy_identity_ok(signature, st) {
        result.signature_valid = false;
        return;
    }
    let ed = &signature[ED_SIG_START..PQ_ID_START];
    let pq = &signature[PQ_SIG_START..];
    let ed_ok = verify_ed25519_signature(message, ed, result, st);
    let pq_ok = verify_mldsa65_signature(message, pq, st);
    result.signature_valid = ed_ok && pq_ok;
}

fn verify_mldsa65_signature(message: &[u8], signature: &[u8], st: &mut SystemTable<Boot>) -> bool {
    print(st, cstr16!("  [CRYPTO] Verifying ML-DSA-65 signature...\r\n"));
    let key = mldsa65::trusted_public_key();
    if mldsa65::verify_signature(key, message, signature) {
        log_info("kernel_verify", "ML-DSA-65 signature VERIFIED against trusted key");
        print(st, cstr16!("  [CRYPTO] ML-DSA-65 verify .................... [PASS]\r\n"));
        true
    } else {
        log_error("kernel_verify", "ML-DSA-65 signature verification FAILED");
        print(st, cstr16!("  [CRYPTO] ML-DSA-65 verify .................... [FAIL]\r\n"));
        false
    }
}
