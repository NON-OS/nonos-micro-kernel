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

//! What the boot log says about side channels: what the part is exposed to,
//! and what this kernel does about it. Kept apart from the code that decides,
//! so the two lists cannot drift into agreeing with each other.

use super::types::{CpuVulnerabilities, MitigationStatus};

pub(super) fn vulnerabilities(v: CpuVulnerabilities) {
    crate::log::info!("[SECURITY] CPU vulnerabilities detected:");
    crate::log::info!("  Spectre v1: {}", v.spectre_v1);
    crate::log::info!("  Spectre v2: {}", v.spectre_v2);
    crate::log::info!("  Spectre v4: {}", v.spectre_v4);
    crate::log::info!("  Meltdown: {}", v.meltdown);
    crate::log::info!("  MDS: {}", v.mds);
    crate::log::info!("  L1TF: {}", v.l1tf);
}

pub(super) fn mitigations(s: MitigationStatus) {
    crate::log::info!("[SECURITY] Mitigations enabled:");
    crate::log::info!("  KPTI: {}", s.kpti_enabled);
    crate::log::info!("  Retpoline: {}", s.retpoline_enabled);
    crate::log::info!("  IBRS: {}", s.ibrs_enabled);
    crate::log::info!("  IBPB: {}", s.ibpb_enabled);
    crate::log::info!("  STIBP: {}", s.stibp_enabled);
    crate::log::info!("  SSBD: {}", s.ssbd_enabled);
    crate::log::info!("  MDS Clear: {}", s.mds_clear_enabled);
    crate::log::info!("  L1D Flush: {}", s.l1d_flush_enabled);
    crate::log::info!("  RSB Stuffing: {}", s.rsb_stuffing_enabled);
}
