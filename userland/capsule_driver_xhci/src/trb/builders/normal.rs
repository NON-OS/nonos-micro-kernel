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
use crate::constants::{TRB_CH, TRB_ISP, TRB_TYPE_NORMAL};
use crate::trb::Trb;
pub fn normal(buffer_phys: u64, length: u32, cycle: bool, ioc: bool, chain: bool) -> Trb {
    let mut trb = Trb::zero();
    trb.set_pointer(buffer_phys);
    trb.set_transfer_length(length);
    trb.set_type(TRB_TYPE_NORMAL);
    trb.d3 |= TRB_ISP;
    if chain {
        trb.d3 |= TRB_CH;
    }
    if ioc {
        trb.set_ioc(true);
    }
    trb.set_cycle(cycle);
    trb
}
