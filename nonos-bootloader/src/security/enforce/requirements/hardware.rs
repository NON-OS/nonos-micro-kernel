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

pub fn enforce_hardware_rng(ctx: &SecurityContext, result: &mut EnforcementResult) {
    if !ctx.hardware_rng_available {
        result.deny("HW RNG required");
        log_error("enforce", "BLOCKED: HW RNG required");
    }
}

pub fn enforce_measured_boot(ctx: &SecurityContext, result: &mut EnforcementResult) {
    if !ctx.measured_boot_active {
        result.deny("TPM required");
        log_error("enforce", "BLOCKED: TPM required");
    }
}
