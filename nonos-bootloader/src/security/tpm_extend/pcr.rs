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

extern crate alloc;

use alloc::format;
use sha2::{Digest, Sha256};
use uefi::prelude::*;

use super::tcg2::{extend_pcr_via_tcg2, locate_tcg2_protocol};
use crate::log::logger::{log_debug, log_error, log_info, log_warn};

pub fn extend_pcr_measurement(st: &mut SystemTable<Boot>, pcr_index: u32, data: &[u8]) -> bool {
    if data.is_empty() {
        log_warn("security", "empty data for PCR");
        return false;
    }
    if pcr_index > 23 {
        log_error("security", "invalid PCR index");
        return false;
    }
    let mut hasher = Sha256::new();
    hasher.update(data);
    let measurement: [u8; 32] = hasher.finalize().into();
    match locate_tcg2_protocol(st.boot_services()) {
        Some(tcg2) => match extend_pcr_via_tcg2(tcg2, pcr_index, &measurement) {
            Ok(()) => {
                log_info("security", &format!("PCR{} extended", pcr_index));
                true
            }
            Err(e) => {
                log_warn("security", &format!("PCR extend failed: {}", e));
                false
            }
        },
        None => {
            log_debug("security", "no TPM2 available");
            false
        }
    }
}
