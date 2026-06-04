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
use super::state::CommandRing;
use crate::constants::{COMMAND_RING_TRBS, TRB_BYTES};
use crate::error::{XhciError, XhciResult};
use crate::trb::builders::link::LinkTrbBuilder;
use crate::trb::{write_volatile_at, Trb};
impl CommandRing {
    pub fn enqueue(&mut self, mut trb: Trb) -> XhciResult<u64> {
        if self.enqueue_index == COMMAND_RING_TRBS - 1 {
            return Err(XhciError::CommandRingFull);
        }
        trb.set_cycle(self.cycle != 0);
        let slot_va = self.region.user_va() + (self.enqueue_index as u64) * (TRB_BYTES as u64);
        let slot_phys = self.region.phys() + (self.enqueue_index as u64) * (TRB_BYTES as u64);
        write_volatile_at(slot_va, trb);
        self.enqueue_index += 1;
        if self.enqueue_index == COMMAND_RING_TRBS - 1 {
            let link = LinkTrbBuilder::new()
                .target(self.region.phys())
                .toggle_cycle(true)
                .cycle(self.cycle != 0)
                .build();
            let link_va =
                self.region.user_va() + ((COMMAND_RING_TRBS as u64) - 1) * (TRB_BYTES as u64);
            write_volatile_at(link_va, link);
            self.cycle ^= 1;
            self.enqueue_index = 0;
        }
        Ok(slot_phys)
    }
}
