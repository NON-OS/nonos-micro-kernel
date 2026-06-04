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
use crate::constants::{TRB_IDT, TRB_TYPE_SETUP_STAGE, TRT_IN_DATA};
use crate::trb::Trb;
pub fn setup_stage_get_descriptor(length: u16, cycle: bool) -> Trb {
    setup_stage_get_descriptor_typed(1, 0, length, cycle)
}
pub fn setup_stage_get_descriptor_typed(
    desc_type: u8,
    desc_index: u8,
    length: u16,
    cycle: bool,
) -> Trb {
    let mut trb = Trb::zero();
    let value = ((desc_type as u16) << 8) | desc_index as u16;
    trb.d0 = 0x80 | (0x06 << 8) | ((value as u32) << 16);
    trb.d1 = (length as u32) << 16;
    trb.d2 = 8 | TRT_IN_DATA;
    trb.d3 = TRB_IDT;
    trb.set_type(TRB_TYPE_SETUP_STAGE);
    trb.set_cycle(cycle);
    trb
}
