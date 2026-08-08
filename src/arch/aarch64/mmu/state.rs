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

use super::table::PageTable;

static mut KERNEL_L0: PageTable = PageTable::new();
static mut KERNEL_L1: PageTable = PageTable::new();
static mut KERNEL_L2: [PageTable; 6] = [
    PageTable::new(),
    PageTable::new(),
    PageTable::new(),
    PageTable::new(),
    PageTable::new(),
    PageTable::new(),
];

/// Level 1 for the high half. TTBR1 walks the same level 0 table as TTBR0, so
/// the direct map hangs off its own level 1 rather than sharing the identity
/// map's.
static mut KERNEL_L1_HIGH: PageTable = PageTable::new();
static mut KERNEL_L3: [[PageTable; 512]; 4] = [[PageTable::new(); 512]; 4];

pub(super) unsafe fn l0() -> &'static mut PageTable {
    &mut KERNEL_L0
}

pub(super) unsafe fn l1() -> &'static mut PageTable {
    &mut KERNEL_L1
}

pub(super) unsafe fn l2(index: usize) -> &'static mut PageTable {
    &mut KERNEL_L2[index]
}

pub(super) unsafe fn l3(l1: usize, l2: usize) -> &'static mut PageTable {
    &mut KERNEL_L3[l1][l2]
}

pub(super) unsafe fn l0_addr() -> u64 {
    &KERNEL_L0 as *const _ as u64
}

pub(super) unsafe fn l1_addr() -> u64 {
    &KERNEL_L1 as *const _ as u64
}

pub(super) unsafe fn l1_high() -> &'static mut PageTable {
    &mut KERNEL_L1_HIGH
}

pub(super) unsafe fn l1_high_addr() -> u64 {
    &KERNEL_L1_HIGH as *const _ as u64
}

pub(super) unsafe fn l2_addr(index: usize) -> u64 {
    &KERNEL_L2[index] as *const _ as u64
}

pub(super) unsafe fn l3_addr(l1: usize, l2: usize) -> u64 {
    &KERNEL_L3[l1][l2] as *const _ as u64
}
