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

use crate::memory::addr::PhysAddr;

use super::super::mmu::{make_satp, mmu_mode, write_satp};




#[inline(always)]
pub(super) unsafe fn switch(root: PhysAddr) {
    let mode = mmu_mode();
    let ppn = (root.as_u64() as usize) >> 12;
    write_satp(make_satp(mode, 0, ppn));
}
