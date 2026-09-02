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

//! What this part is vulnerable to.
//!
//! Everything starts vulnerable and is only ever cleared by a positive
//! statement from the hardware. A CPU too old to have ARCH_CAPABILITIES says
//! nothing, and the honest reading of silence is "assume affected" — the
//! opposite default would report an unknown part as safe.

use super::constants::{
    ARCH_CAP_MDS_NO, ARCH_CAP_RDCL_NO, ARCH_CAP_SBDR_SSDP_NO, ARCH_CAP_SSB_NO, ARCH_CAP_TAA_NO,
    MSR_IA32_ARCH_CAPABILITIES,
};
use super::cpuid;
use super::msr::rdmsr;
use super::types::CpuVulnerabilities;

pub fn detect_vulnerabilities() -> CpuVulnerabilities {
    let mut vulns = CpuVulnerabilities::default();

    if cpuid::has_arch_capabilities() {
        // SAFETY: ek@nonos.systems - CPUID reported ARCH_CAPABILITIES, so this
        // MSR exists on this part and reading it has no side effect.
        let caps = unsafe { rdmsr(MSR_IA32_ARCH_CAPABILITIES) };

        if caps & ARCH_CAP_RDCL_NO != 0 {
            vulns.meltdown = false;
        }
        if caps & ARCH_CAP_SSB_NO != 0 {
            vulns.spectre_v4 = false;
        }
        if caps & ARCH_CAP_MDS_NO != 0 {
            vulns.mds = false;
        }
        if caps & ARCH_CAP_TAA_NO != 0 {
            vulns.taa = false;
        }
        if caps & ARCH_CAP_SBDR_SSDP_NO != 0 {
            vulns.srbds = false;
        }
    }

    // Neither Meltdown nor MDS has ever affected an AMD part, and AMD does not
    // report the ARCH_CAPABILITIES bits that would say so above.
    if cpuid::is_amd() {
        vulns.meltdown = false;
        vulns.mds = false;
    }

    vulns
}
