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

use crate::boot::util::fatal_reset;
use crate::display::{log_ok, show_error_screen};
use crate::image_format::{has_production_footer, parse_image_footer};
use crate::log::logger::{log_error, log_info};
use alloc::format;

use crate::menu::SecurityMode;
use crate::security::{check_kernel_version, read_floor};

pub fn check_rollback(st: &mut SystemTable<Boot>, data: &[u8], mode: SecurityMode, gop: bool) {
    if !has_production_footer(data) {
        return;
    }
    let parsed = match parse_image_footer(data) {
        Ok(parsed) => parsed,
        Err(_) => return,
    };
    // Gate on rollback_index, the field bound into the kernel signature
    // (hash || rollback_index). image_version is not signed, so a replayed
    // old-but-validly-signed kernel could carry any image_version and defeat
    // the NVRAM floor; rollback_index cannot be altered without breaking the
    // signature that was already verified before this runs. This also keeps
    // the NVRAM floor and the TPM floor on the same monotonic counter.
    let rollback_index = parsed.footer.rollback_index as u64;
    match check_kernel_version(rollback_index) {
        Ok(()) => {
            log_info("rollback", "kernel version acceptable");
            if gop {
                log_ok(b"Anti-rollback check PASSED");
            }
        }
        Err(e) => {
            if mode.requires_signature() {
                log_error("rollback", "kernel version rollback detected");
                if gop {
                    // The reason and numbers make a photo of this screen a
                    // full diagnosis on machines with no serial console.
                    let msg = format!("Rollback check failed: {} index {}", e.as_str(), rollback_index);
                    show_error_screen(msg.as_bytes());
                }
                fatal_reset(st, e.as_str());
            }
            log_info("rollback", "rollback detected but dev mode - continuing");
        }
    }
    if let Some(floor) = read_floor(st.boot_services()) {
        log_info("rollback", &format!("tpm floor {} index {}", floor, rollback_index));
        if rollback_index < floor && mode.requires_signature() {
            log_error("rollback", "rollback index below TPM monotonic floor");
            if gop {
                let msg =
                    format!("Rollback: tpm floor {} above image index {}", floor, rollback_index);
                show_error_screen(msg.as_bytes());
            }
            fatal_reset(st, "rollback index below TPM floor");
        }
    }
}
