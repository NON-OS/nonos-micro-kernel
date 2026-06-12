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
use crate::log::logger::{log_error, log_info};
use crate::menu::SecurityMode;
use crate::security::{check_kernel_version, update_kernel_version};

use super::super::util::fatal_reset;

pub fn check_rollback(st: &mut SystemTable<Boot>, data: &[u8], mode: SecurityMode, gop: bool) {
    if !has_production_footer(data) {
        return;
    }
    let version = match parse_image_footer(data) {
        Ok(parsed) => parsed.footer.image_version as u64,
        Err(_) => return,
    };
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
}

pub fn commit_rollback(st: &mut SystemTable<Boot>, data: &[u8], mode: SecurityMode, gop: bool) {
    if !has_production_footer(data) {
        return;
    }
    let version = match parse_image_footer(data) {
        Ok(parsed) => parsed.footer.image_version as u64,
        Err(e) => {
            if mode.requires_signature() {
                log_error("rollback", "kernel version footer parse failed");
                fatal_reset(st, e.as_str());
            }
            return;
        }
    };
    let timestamp = get_uefi_time_epoch(st);
    match update_kernel_version(version, timestamp) {
        Ok(()) => {
            log_info("rollback", "kernel version committed");
            if gop {
                log_ok(b"Anti-rollback commit PASSED");
            }
        }
        Err(e) => {
            if mode.requires_signature() {
                log_error("rollback", "kernel version commit failed");
                if gop {
                    show_error_screen(b"Rollback state update failed");
                }
                fatal_reset(st, e.as_str());
            }
            log_info("rollback", "rollback commit failed but dev mode - continuing");
        }
    }
}
