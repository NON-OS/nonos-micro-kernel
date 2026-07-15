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
use super::super::ops_helpers::iso_flags_for;
use super::super::ops_msi::is_gsi_claimed;
use super::super::state::VEC_ALLOC;
use super::super::types::{IsoFlags, Rte};

pub fn alloc_route(gsi: u32, dest_apic_id: u32) -> IoApicResult<(u8, Rte)> {
    if is_gsi_claimed(gsi) {
        return Err(IoApicError::GsiClaimedForMsi);
    }
    let vector = VEC_ALLOC.lock().alloc().ok_or(IoApicError::VectorExhausted)?;
    let mut rte = Rte::fixed(vector, dest_apic_id);
    if let Some(flags) = iso_flags_for(gsi) {
        if flags.contains(IsoFlags::TRIGGER_LEVEL) {
            rte.level_trigger = true;
        }
        if flags.contains(IsoFlags::POLARITY_ACTIVE_LOW) {
            rte.active_low = true;
        }
    }
    Ok((vector, rte))
}
