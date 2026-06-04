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
use crate::constants::{COMMAND_RING_TRBS, TRB_BYTES};
use crate::dma::{DmaPool, DmaRegion};
use crate::error::XhciResult;
use crate::trb::builders::link::LinkTrbBuilder;
use crate::trb::write_volatile_at;
pub struct TransferRing {
    pub(super) region: DmaRegion,
    pub(super) cycle: u8,
    pub(super) enqueue_index: usize,
}
impl TransferRing {
    pub fn new(pool: &DmaPool) -> XhciResult<Self> {
        let bytes = (COMMAND_RING_TRBS as u64) * (TRB_BYTES as u64);
        let region = pool.alloc(bytes)?;
        region.zero();
        let link_va = region.user_va() + ((COMMAND_RING_TRBS as u64) - 1) * (TRB_BYTES as u64);
        let link =
            LinkTrbBuilder::new().target(region.phys()).toggle_cycle(true).cycle(true).build();
        write_volatile_at(link_va, link);
        Ok(Self { region, cycle: 1, enqueue_index: 0 })
    }
}
