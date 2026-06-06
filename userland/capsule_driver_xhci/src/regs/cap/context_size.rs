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
use crate::constants::HCCPARAMS1;
use crate::regs::mmio_read32;
const HCCPARAMS1_CSZ: u32 = 1 << 2;
pub fn context_size(mmio_base: u64) -> u8 {
    if (mmio_read32(mmio_base + HCCPARAMS1) & HCCPARAMS1_CSZ) != 0 {
        64
    } else {
        32
    }
}
