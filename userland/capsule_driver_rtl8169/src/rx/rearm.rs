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

use core::sync::atomic::{compiler_fence, Ordering};

use crate::constants::queue::{BUFFER_SIZE, RX_DESC_COUNT};
use crate::constants::regs::{DESC_EOR, DESC_OWN};
use crate::queue::desc::{desc_mut, Descriptor};
use crate::setup::Driver;

pub(super) fn rearm(driver: &Driver, idx: usize) {
    let eor = if idx == RX_DESC_COUNT - 1 { DESC_EOR } else { 0 };
    let addr = driver.rx.buffer_da(idx);
    let d = Descriptor {
        opts1: DESC_OWN | eor | BUFFER_SIZE as u32,
        opts2: 0,
        addr_lo: addr as u32,
        addr_hi: (addr >> 32) as u32,
    };
    compiler_fence(Ordering::Release);
    unsafe {
        desc_mut(driver.rx.desc_va, idx, d);
    }
}
