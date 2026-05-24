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
use crate::constants::{EVENT_RING_SEGMENT_TABLE_ENTRIES, IMAN_IE};
use crate::regs::runtime::{erdp_program, erstba_program, erstsz_program, iman_read, iman_write};
use crate::rings::event::EventRing;
pub fn program_event_ring(intr_base: u64, ring: &EventRing) {
    erstsz_program(intr_base, EVENT_RING_SEGMENT_TABLE_ENTRIES as u16);
    erdp_program(intr_base, ring.current_dequeue_phys(), false, 0);
    erstba_program(intr_base, ring.erst_base_phys());
    let cur = iman_read(intr_base);
    iman_write(intr_base, cur | IMAN_IE);
}