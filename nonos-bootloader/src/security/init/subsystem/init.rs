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
use crate::hardware::tpm::init_tpm;
use crate::security::crypto::{blake3_health_check, ed25519_health_check};
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
    ctx.blake3_health_ok = blake3_health_check();
    ctx.ed25519_health_ok = ed25519_health_check();
    ctx.measured_boot_active = check_measured_boot(st);
    // Bring up the register-level TPM as well. The check above extends a PCR
    // through firmware, which proves the TPM answers but leaves the MMIO state
    // machine undetected, and that is the one the endorsement key and the
    // anti-rollback NV counter both go through. init_tpm had no caller at all,
    // so the EK read failed with "TPM not initialized" every boot and the
    // machine id fell back to software without anything being wrong with the
    // hardware.
    let _ = init_tpm();
    display_security_status(&ctx, st);
    ctx
}
