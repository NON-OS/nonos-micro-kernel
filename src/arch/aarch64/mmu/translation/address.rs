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

use super::stage::translate_stage1_read;

const PHYS_OFFSET: u64 = 0xFFFF_0000_0000_0000;
const KERNEL_OFFSET: u64 = 0xFFFF_8000_0000_0000;

pub fn virt_to_phys(virt: u64) -> Option<u64> {
    if virt >= KERNEL_OFFSET {
        return Some(virt - KERNEL_OFFSET);
    }
    if virt >= PHYS_OFFSET {
        return Some(virt - PHYS_OFFSET);
    }
    translate_stage1_read(virt).ok()
}

pub fn phys_to_virt(phys: u64) -> u64 {
    PHYS_OFFSET + phys
}

pub fn kernel_phys_to_virt(phys: u64) -> u64 {
    KERNEL_OFFSET + phys
}
