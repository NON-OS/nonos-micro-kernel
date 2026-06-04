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
use crate::constants::TRB_TYPE_DISABLE_SLOT_CMD;
use crate::trb::Trb;
const SLOT_ID_SHIFT: u32 = 24;
pub fn disable_slot_command(cycle: bool, slot_id: u8) -> Trb {
    let mut trb = Trb::zero();
    trb.set_type(TRB_TYPE_DISABLE_SLOT_CMD);
    trb.set_cycle(cycle);
    trb.d3 |= (slot_id as u32) << SLOT_ID_SHIFT;
    trb
}
