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

use crate::log::logger::log_error;
use crate::security::enforce::policy::EnforcementResult;
use crate::security::types::SecurityContext;

pub fn enforce_secure_boot(ctx: &SecurityContext, result: &mut EnforcementResult) {
    if !ctx.secure_boot_enabled {
        result.deny("SecureBoot required");
        log_error("enforce", "BLOCKED: SecureBoot required");
    }
}

pub fn enforce_platform_key(ctx: &SecurityContext, result: &mut EnforcementResult) {
    if !ctx.platform_key_verified {
        result.deny("PlatformKey required");
        log_error("enforce", "BLOCKED: PlatformKey required");
    }
}

pub fn enforce_signature_db(ctx: &SecurityContext, result: &mut EnforcementResult) {
    if !ctx.signature_database_valid {
        result.deny("SignatureDB required");
        log_error("enforce", "BLOCKED: SignatureDB required");
    }
}
