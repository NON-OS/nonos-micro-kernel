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
    verify_and_display_signature(parsed.kernel_bytes, parsed.signature_bytes, &mut result, st);
    if result.signature_valid {
        print_verification_success(st);
    } else {
        print_verification_failure(st);
    }

    log_info("kernel_verify", "Cryptographic verification complete");
    result
}
