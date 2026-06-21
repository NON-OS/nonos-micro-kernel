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

pub fn enforce_crypto_health(ctx: &SecurityContext, result: &mut EnforcementResult) {
    if !ctx.blake3_health_ok {
        result.deny("BLAKE3 health check failed");
        log_error("enforce", "BLAKE3 health check FAILED");
    }
    if !ctx.ed25519_health_ok {
        result.deny("Ed25519 health check failed");
        log_error("enforce", "Ed25519 health check FAILED");
    }
}

pub fn enforce_keys_loaded(ctx: &SecurityContext, result: &mut EnforcementResult) {
    if !ctx.production_keys_loaded {
        result.deny("signing keys not loaded");
        log_error("enforce", "no signing keys");
    }
    if ctx.key_count == 0 {
        result.deny("zero signing keys");
        log_error("enforce", "key count is zero");
    }
}
