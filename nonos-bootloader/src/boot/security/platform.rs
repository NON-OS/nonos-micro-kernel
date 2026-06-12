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

use crate::display::{log_info as panel_info, log_ok};
use crate::security::{audit, init_anti_rollback, init_attestation, initialize_security_subsystem};
use crate::security::{
    verify_platform_security, AuditEvent, HardwareCapabilities, SecurityContext,
};

pub fn verify_platform(hw_caps: &HardwareCapabilities, gop: bool) {
    let platform = verify_platform_security(hw_caps);
    if gop {
        if platform.exploit_mitigations {
            log_ok(b"CPU mitigations: SMEP+SMAP+NX");
        }
        if platform.hardware_rng {
            log_ok(b"Hardware RNG available");
        }
        if platform.tpm_attestation {
            log_ok(b"TPM2 attestation ready");
        }
    }
}

pub fn init_subsystems(st: &mut SystemTable<Boot>, gop: bool) -> SecurityContext {
    let security = initialize_security_subsystem(st);
    init_attestation();
    let _ = init_anti_rollback(security.measured_boot_active);
    audit(AuditEvent::TpmInit, 0, b"subsystems ready");
    if gop {
        display_subsystem_status(&security);
    }
    security
}

fn display_subsystem_status(security: &SecurityContext) {
    if security.secure_boot_enabled {
        log_ok(b"SecureBoot ENABLED");
    } else {
        panel_info(b"SecureBoot disabled");
    }
    if security.measured_boot_active {
        log_ok(b"TPM2 MeasuredBoot active");
    } else {
        panel_info(b"TPM2 not available");
    }
}
