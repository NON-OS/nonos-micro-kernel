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
use super::state::EventRing;
use crate::constants::{EVENT_RING_SEGMENT_TRBS, TRB_BYTES};
use crate::trb::{write_volatile_at, Trb};
impl EventRing {
    pub fn advance(&mut self) {
        let va = self.segment.user_va() + (self.dequeue_index as u64) * (TRB_BYTES as u64);
        write_volatile_at(va, Trb::zero());
        self.dequeue_index += 1;
        if self.dequeue_index == EVENT_RING_SEGMENT_TRBS {
            self.dequeue_index = 0;
            self.consumer_cycle ^= 1;
        }
    }
}
