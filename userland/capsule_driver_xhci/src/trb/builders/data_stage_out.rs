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
use crate::constants::TRB_TYPE_DATA_STAGE;
use crate::trb::Trb;
pub fn data_stage_out(buffer_phys: u64, length: u16, cycle: bool) -> Trb {
    let mut trb = Trb::zero();
    trb.set_pointer(buffer_phys);
    trb.set_transfer_length(length as u32);
    trb.set_type(TRB_TYPE_DATA_STAGE);
    trb.set_cycle(cycle);
    trb
}
