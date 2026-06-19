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

use super::output::output_status;
use crate::security::types::SecurityContext;
use uefi::cstr16;
use uefi::prelude::*;

pub fn display_security_status(ctx: &SecurityContext, st: &mut SystemTable<Boot>) {
    let _ = st.stdout().output_string(cstr16!("=== Security Status ===\r\n"));
    output_status(st, "Production Keys", ctx.production_keys_loaded);
    output_status(st, "SecureBoot", ctx.secure_boot_enabled);
    output_status(st, "PlatformKey", ctx.platform_key_verified);
    output_status(st, "SignatureDB", ctx.signature_database_valid);
    output_status(st, "HW RNG", ctx.hardware_rng_available);
    output_status(st, "Measured Boot", ctx.measured_boot_active);
    output_status(st, "Ed25519", ctx.ed25519_health_ok);
    output_status(st, "BLAKE3", ctx.blake3_health_ok);
    let _ = st.stdout().output_string(cstr16!("=======================\r\n"));
}
