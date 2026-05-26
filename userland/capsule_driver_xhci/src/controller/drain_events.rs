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
use crate::constants::{TRB_TYPE_CMD_COMPLETION_EVENT, TRB_TYPE_TRANSFER_EVENT};
use crate::regs::runtime::erdp_program;
use crate::rings::event::EventRing;
use crate::trb::Trb;
pub const DRAIN_BATCH: usize = 32;
pub struct DrainBatch {
    pub trbs: [Trb; DRAIN_BATCH],
    pub count: usize,
}
impl DrainBatch {
    pub fn new() -> Self {
        Self { trbs: [Trb::zero(); DRAIN_BATCH], count: 0 }
    }
}
pub fn drain_events(intr_base: u64, ring: &mut EventRing) -> DrainBatch {
    let mut batch = DrainBatch::new();
    while batch.count < DRAIN_BATCH && ring.has_event() {
        let trb = ring.current_trb();
        if reserved_for_waiter(trb.get_type()) {
            break;
        }
        batch.trbs[batch.count] = trb;
        batch.count += 1;
        ring.advance();
    }
    erdp_program(intr_base, ring.current_dequeue_phys(), true, 0);
    batch
}
fn reserved_for_waiter(trb_type: u32) -> bool {
    trb_type == TRB_TYPE_TRANSFER_EVENT || trb_type == TRB_TYPE_CMD_COMPLETION_EVENT
}