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

use super::super::error::{IoApicError, IoApicResult};
use super::super::gsi_owners;
use super::super::ops_helpers::iso_flags_for;
use super::super::ops_msi::is_gsi_claimed;
use super::super::types::{IsoFlags, Rte};
use super::program_route::program_route;

// Program an IO-APIC redirection entry with a caller-supplied
// vector. The vector must be allocated by the caller (the driver
// broker manages its own pool over the reserved 0x60..=0x6F range);
// this helper does the GSI ownership CAS (Free -> Capsule), the
// RTE construction, and the MMIO write. The route is built
// level-triggered and active-low when the MADT ISO table says so,
// mirroring `alloc_route`. The CAS rolls back if `program_route`
// fails so a partial MMIO failure cannot strand an owner bit.
pub fn program_route_external(gsi: u32, vector: u8, dest_apic_id: u32) -> IoApicResult<Rte> {
    if is_gsi_claimed(gsi) {
        return Err(IoApicError::GsiClaimedForMsi);
    }
    gsi_owners::claim_for_capsule(gsi)?;
    let mut rte = Rte::fixed(vector, dest_apic_id);
    if let Some(flags) = iso_flags_for(gsi) {
        if flags.contains(IsoFlags::TRIGGER_LEVEL) {
            rte.level_trigger = true;
        }
        if flags.contains(IsoFlags::POLARITY_ACTIVE_LOW) {
            rte.active_low = true;
        }
    }
    if let Err(e) = program_route(gsi, rte) {
        let _ = gsi_owners::release_capsule(gsi);
        return Err(e);
    }
    Ok(rte)
}
