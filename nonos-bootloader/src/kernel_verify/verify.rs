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

use uefi::prelude::*;
use super::delay::mini_delay;
use super::display_status::{
    print_kernel_size, print_verification_failure, print_verification_success,
};
use super::footer::handle_missing_footer;
use super::helpers::{
    compute_and_display_hash, initialize_crypto_if_needed, validate_kernel_size,
    verify_and_display_signature,
};
use super::types::CryptoVerifyResult;
use crate::image_format::{has_production_footer, validate_image};
use crate::log::logger::{log_error, log_info};

pub fn verify_kernel_crypto(kernel_data: &[u8], st: &mut SystemTable<Boot>) -> CryptoVerifyResult {
    log_info("kernel_verify", "Starting cryptographic verification");
    let mut result = CryptoVerifyResult::new();
    if !initialize_crypto_if_needed(st) {
        log_error("kernel_verify", "Crypto initialization failed");
        return result;
    }
    if !validate_kernel_size(kernel_data, st) {
        log_error("kernel_verify", "Kernel size validation failed");
        return result;
    }
    if !has_production_footer(kernel_data) {
        handle_missing_footer(kernel_data, &mut result, st);
        return result;
    }
    let parsed = match validate_image(kernel_data) {
        Ok(p) => p,
        Err(e) => {
            log_error("kernel_verify", "Image validation failed");
            log_error("kernel_verify", e.as_str());
            compute_and_display_hash(kernel_data, &mut result, st);
            return result;
        }
    };
    result.kernel_code_size = parsed.kernel_bytes.len();
    result.signature_present = true;
    print_kernel_size(st, parsed.kernel_bytes.len());
    mini_delay();
    compute_and_display_hash(parsed.kernel_bytes, &mut result, st);
    let kernel_hash = result.kernel_hash_full;
    verify_and_display_signature(
        &kernel_hash,
        parsed.footer.rollback_index,
        parsed.signature_algorithm,
        parsed.signature_bytes,
        &mut result,
        st,
    );
    if result.signature_valid {
        print_verification_success(st);
    } else {
        print_verification_failure(st);
    }
    #[cfg(feature = "stark-kernel-attest")]
    verify_kernel_stark_self_attestation(&parsed, &mut result);
    log_info("kernel_verify", "Cryptographic verification complete");
    result
}

/// Verify the kernel's transparent STARK self-attestation carried in the proof
/// footer, against the enrolled boot root. A kernel that ships a trailer must
/// prove its measurement; one that does not is left to signature trust alone.
#[cfg(feature = "stark-kernel-attest")]
fn verify_kernel_stark_self_attestation(
    parsed: &crate::image_format::ParsedImage<'_>,
    result: &mut CryptoVerifyResult,
) {
    const MAGIC: &[u8; 8] = b"NZKSTRK1";
    let Some(trailer) = parsed.proof_bytes else {
        log_info("kernel_verify", "no STARK self-attestation trailer present");
        return;
    };
    if trailer.len() < 8 || &trailer[0..8] != MAGIC {
        return;
    }
    if super::stark_attest::verify_kernel_self_attestation(parsed.kernel_bytes, trailer) {
        result.stark_attested = true;
        log_info("kernel_verify", "kernel STARK self-attestation verified");
    } else {
        result.stark_attested = false;
        log_error("kernel_verify", "kernel STARK self-attestation FAILED");
    }
}
