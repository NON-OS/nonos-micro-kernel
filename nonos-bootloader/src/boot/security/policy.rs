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

use crate::display::{log_ok, show_error_screen, update_stage, StageStatus, STAGE_SECURITY};
use crate::log::logger::{log_error, log_warn};
use crate::menu::SecurityMode;
use crate::security::{
    enforce_security_policy, verify_secure_boot_chain, SecurityContext, SecurityPolicy,
};

use super::super::util::fatal_reset;

pub fn enforce_policy(
    security: &SecurityContext,
    st: &mut SystemTable<Boot>,
    gop: bool,
    mode: SecurityMode,
) {
    let enforcement = enforce_security_policy(security, st, mode);
    if !enforcement.allow_boot {
        log_error("security", enforcement.reason);
        update_stage(STAGE_SECURITY, StageStatus::Failed);
        if gop {
            show_error_screen(b"Security policy enforcement failed");
        }
        fatal_reset(st, enforcement.reason);
    }
    if gop {
        log_ok(b"Security policy: ALLOW_BOOT");
    }
}

pub fn verify_chain(security: &SecurityContext, st: &mut SystemTable<Boot>) {
    if security.secure_boot_enabled && !verify_secure_boot_chain(security, st) {
        if SecurityPolicy::from_build() == SecurityPolicy::Development {
            log_warn("security", "Secure Boot chain verification warning");
        } else {
            log_error("security", "Secure Boot chain verification failed");
            fatal_reset(st, "Secure Boot chain verification failed");
        }
    }
}
