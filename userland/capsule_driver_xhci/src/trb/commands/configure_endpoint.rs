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
use crate::constants::TRB_TYPE_CONFIGURE_ENDPOINT_CMD;
use crate::trb::Trb;
pub fn configure_endpoint_command(input_context_phys: u64, slot_id: u8, cycle: bool) -> Trb {
    let mut trb = Trb::zero();
    trb.set_pointer(input_context_phys);
    trb.set_type(TRB_TYPE_CONFIGURE_ENDPOINT_CMD);
    trb.d3 |= (slot_id as u32) << 24;
    trb.set_cycle(cycle);
    trb
}
