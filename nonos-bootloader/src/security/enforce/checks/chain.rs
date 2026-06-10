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

use super::bypass::detect_secure_boot_bypass;
use crate::log::logger::{log_info, log_warn};
use crate::security::types::SecurityContext;
use uefi::prelude::*;

pub fn verify_secure_boot_chain(
    ctx: &SecurityContext,
    system_table: &mut SystemTable<Boot>,
) -> bool {
    if !ctx.secure_boot_enabled {
        return true;
    }
    if detect_secure_boot_bypass(system_table) {
        log_warn("enforce", "SecureBoot bypass detected");
        return false;
    }
    if !ctx.platform_key_verified {
        log_warn("enforce", "PlatformKey not verified");
        return false;
    }
    if !ctx.signature_database_valid {
        log_warn("enforce", "SignatureDB not valid");
        return false;
    }
    log_info("enforce", "SecureBoot chain verified");
    true
}
