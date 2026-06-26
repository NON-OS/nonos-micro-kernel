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

use crate::display::{log_ok, show_error_screen};
use crate::handoff::get_uefi_time_epoch;
use crate::image_format::{has_production_footer, parse_image_footer};
use crate::log::logger::{log_error, log_info, log_warn};
use alloc::format;

use crate::menu::SecurityMode;
use crate::security::{check_kernel_version, commit_floor, read_floor, update_kernel_version};

use super::super::util::fatal_reset;

pub fn check_rollback(st: &mut SystemTable<Boot>, data: &[u8], mode: SecurityMode, gop: bool) {
    if !has_production_footer(data) {
        return;
    }
    let parsed = match parse_image_footer(data) {
        Ok(parsed) => parsed,
        Err(_) => return,
    };
    let version = parsed.footer.image_version as u64;
    let rollback_index = parsed.footer.rollback_index as u64;
    match check_kernel_version(version) {
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
                    show_error_screen(b"Rollback attack detected");
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
                show_error_screen(b"Rollback attack detected");
            }
            fatal_reset(st, "rollback index below TPM floor");
        }
    }
}

pub fn commit_rollback(st: &mut SystemTable<Boot>, data: &[u8], mode: SecurityMode, gop: bool) {
    if !has_production_footer(data) {
        return;
    }
    let parsed = match parse_image_footer(data) {
        Ok(parsed) => parsed,
        Err(e) => {
            if mode.requires_signature() {
                log_error("rollback", "kernel version footer parse failed");
                fatal_reset(st, e.as_str());
            }
            return;
        }
    };
    let version = parsed.footer.image_version as u64;
    let rollback_index = parsed.footer.rollback_index as u64;
    let timestamp = get_uefi_time_epoch(st);
    match update_kernel_version(version, timestamp) {
        Ok(()) => {
            log_info("rollback", "kernel version committed");
            if gop {
                log_ok(b"Anti-rollback commit PASSED");
            }
        }
        Err(_) => {
            log_warn(
                "rollback",
                "legacy nvram commit unavailable; enforcing via TPM counter floor",
            );
        }
    }
    if commit_floor(st.boot_services(), rollback_index) {
        log_info("rollback", "tpm rollback floor committed");
    }
}
