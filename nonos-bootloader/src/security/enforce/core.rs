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
use uefi::prelude::*;

use super::modes::{enforce_development, enforce_hardened, enforce_standard};
use super::policy::{EnforcementResult, SecurityPolicy};
use super::requirements::{enforce_crypto_health, enforce_keys_loaded};
use crate::display::display_enforcement_result;
use crate::log::logger::log_info;
use crate::menu::SecurityMode;
use crate::security::types::SecurityContext;

fn policy_for_mode(mode: SecurityMode) -> SecurityPolicy {
    match mode {
        SecurityMode::Development => SecurityPolicy::Development,
        SecurityMode::Standard | SecurityMode::SafeMode | SecurityMode::Recovery => {
            SecurityPolicy::Standard
        }
        SecurityMode::Hardened | SecurityMode::NetworkIsolated => SecurityPolicy::Hardened,
    }
}

pub fn enforce_security_policy(
    ctx: &SecurityContext,
    system_table: &mut SystemTable<Boot>,
    mode: SecurityMode,
) -> EnforcementResult {
    // The compile-time policy is a floor that the menu can only raise, never
    // lower, so a console operator cannot downgrade a hardened image.
    let floor = SecurityPolicy::from_build();
    let requested = policy_for_mode(mode);
    let policy = floor.stricter(requested);
    let mut result = EnforcementResult::new(policy);

    log_info(
        "enforce",
        &format!("policy: {:?} (build floor {:?}, menu {:?})", policy, floor, requested),
    );

    enforce_crypto_health(ctx, &mut result);
    enforce_keys_loaded(ctx, &mut result);

    match policy {
        SecurityPolicy::Development => enforce_development(ctx, &mut result),
        SecurityPolicy::Standard => enforce_standard(ctx, &mut result),
        SecurityPolicy::Hardened => enforce_hardened(ctx, &mut result),
    }

    display_enforcement_result(&result, system_table);
    result
}
