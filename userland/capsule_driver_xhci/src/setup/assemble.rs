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

use super::driver::Driver;
use crate::controller::{ControllerLayout, Scratchpads};
use crate::dma::{DmaPool, DmaRegion};
use crate::handles::BrokerHandles;
use crate::rings::{command::CommandRing, event::EventRing};
use crate::slots::SlotTable;

pub(super) fn assemble(
    handles: BrokerHandles,
    dcbaa: DmaRegion,
    scratchpads: Scratchpads,
    dma_pool: DmaPool,
    command_ring: CommandRing,
    event_ring: EventRing,
    layout: ControllerLayout,
) -> Driver {
    Driver {
        handles,
        dcbaa,
        scratchpads,
        dma_pool,
        command_ring,
        event_ring,
        layout,
        slots: SlotTable::new(),
    }
}
