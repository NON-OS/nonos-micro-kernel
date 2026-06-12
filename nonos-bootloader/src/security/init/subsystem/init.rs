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

use super::keys::load_production_keys;
use crate::security::check::{
    check_hardware_rng, check_measured_boot, check_platform_key, check_secure_boot,
    check_signature_db,
};
use crate::security::crypto::{blake3_selftest, ed25519_selftest};
use crate::security::init::display::display_security_status;
use crate::security::types::SecurityContext;
use uefi::prelude::*;

pub fn initialize_security_subsystem(st: &mut SystemTable<Boot>) -> SecurityContext {
    let mut ctx = SecurityContext::new();
    ctx.production_keys_loaded = load_production_keys(&mut ctx);
    ctx.secure_boot_enabled = check_secure_boot(st);
    ctx.platform_key_verified = check_platform_key(st);
    ctx.signature_database_valid = check_signature_db(st);
    ctx.hardware_rng_available = check_hardware_rng(st);
    ctx.blake3_selftest_ok = blake3_selftest();
    ctx.ed25519_selftest_ok = ed25519_selftest();
    ctx.measured_boot_active = check_measured_boot(st);
    display_security_status(&ctx, st);
    ctx
}
