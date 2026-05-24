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
use crate::controller::{ControllerLayout, Scratchpads};
use crate::dma::{DmaPool, DmaRegion};
use crate::handles::BrokerHandles;
use crate::rings::command::CommandRing;
use crate::rings::event::EventRing;
use crate::slots::SlotTable;
pub struct Driver {
    pub handles: BrokerHandles,
    pub dcbaa: DmaRegion,
    pub scratchpads: Scratchpads,
    pub dma_pool: DmaPool,
    pub command_ring: CommandRing,
    pub event_ring: EventRing,
    pub layout: ControllerLayout,
    pub slots: SlotTable,
}