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

//! Turning on what the part supports, and reporting only what was turned on.
//!
//! A field here means "this kernel does this", never "this CPU could". The
//! two read the same in a log and are not the same thing, and the second one
//! is how a machine ends up trusted for a mitigation nobody wired up.

use super::cpuid;
use super::ibrs::ibrs_enable;
use super::ssbd::ssbd_enable;
use super::stibp::stibp_enable;
use super::types::MitigationStatus;

pub fn enable_mitigations() -> MitigationStatus {
    let mut status = MitigationStatus::default();

    if cpuid::has_ibrs_ibpb() {
        ibrs_enable();
        status.ibrs_enabled = true;
        status.ibpb_enabled = true;
    }
    if cpuid::has_stibp() {
        stibp_enable();
        status.stibp_enabled = true;
    }
    if cpuid::has_ssbd() {
        ssbd_enable();
        status.ssbd_enabled = true;
    }

    // These two are buffer-clearing operations rather than modes: the entry
    // and exit hooks issue VERW and the L1D flush, and both check the same
    // CPUID bit before doing so. Recording support here is therefore also
    // recording that the hook will act.
    status.mds_clear_enabled = cpuid::has_md_clear();
    status.l1d_flush_enabled = cpuid::has_l1d_flush();

    // Unconditional: the hooks refill the return stack buffer with no feature
    // check, and both sides of the privilege boundary call them.
    status.rsb_stuffing_enabled = true;

    // KPTI stays false because this kernel does not implement it. It was
    // reported from CR4.PCIDE, which is process-context identifiers and has
    // nothing to do with unmapping the kernel from the user page table, so a
    // machine with PCID and no KPTI read back as protected. Parts affected by
    // Meltdown are named by `detect_vulnerabilities` and are not mitigated
    // here; saying that plainly is worth more than a field that agrees.
    status
}
