// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
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

//! Bring-up: find out what this part is exposed to, turn on what it offers,
//! and latch both so the rest of the kernel can ask later.

use core::sync::atomic::Ordering;

use super::detect::detect_vulnerabilities;
use super::enable::enable_mitigations;
use super::report;
use super::state::{CPU_VULNERABILITIES, INITIALIZED, MITIGATIONS_ENABLED, MITIGATION_STATUS};
use super::types::{CpuVulnerabilities, MitigationStatus};

pub fn init() -> Result<(), &'static str> {
    if INITIALIZED.swap(true, Ordering::SeqCst) {
        return Ok(());
    }

    crate::log::info!("[SECURITY] Initializing side-channel mitigations...");

    let vulns = detect_vulnerabilities();
    // SAFETY: ek@nonos.systems - the swap above makes this the only path that
    // ever reaches here, and it runs before any other CPU is started.
    unsafe {
        CPU_VULNERABILITIES = vulns;
    }
    report::vulnerabilities(vulns);

    let status = enable_mitigations();
    // SAFETY: ek@nonos.systems - as above; single initialisation path.
    unsafe {
        MITIGATION_STATUS = status;
    }
    report::mitigations(status);

    MITIGATIONS_ENABLED.store(true, Ordering::SeqCst);
    Ok(())
}

pub fn get_vulnerabilities() -> CpuVulnerabilities {
    // SAFETY: ek@nonos.systems - written once during init, read-only after.
    unsafe { CPU_VULNERABILITIES }
}

pub fn get_mitigation_status() -> MitigationStatus {
    // SAFETY: ek@nonos.systems - written once during init, read-only after.
    unsafe { MITIGATION_STATUS }
}

pub fn are_mitigations_enabled() -> bool {
    MITIGATIONS_ENABLED.load(Ordering::SeqCst)
}
