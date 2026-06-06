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
use crate::constants::{ERDP_EHB, ERDP_LO};
use crate::regs::mmio_write64;
pub fn erdp_program(intr_base: u64, dequeue_phys: u64, ehb_clear: bool, desi: u8) {
    let mut value = dequeue_phys & !0xF;
    value |= (desi as u64) & 0x7;
    if ehb_clear {
        value |= ERDP_EHB;
    }
    mmio_write64(intr_base + ERDP_LO, value);
}
