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

use crate::constants::regs::DESC_OWN;
use crate::queue::desc::desc;
use crate::setup::Driver;

const TX_POLL_BUDGET: u32 = 1_000_000;

pub(super) fn poll_done(driver: &Driver, idx: usize) -> Result<(), &'static str> {
    for _ in 0..TX_POLL_BUDGET {
        compiler_fence(Ordering::Acquire);
        if (unsafe { desc(driver.tx.desc_va, idx) }.opts1 & DESC_OWN) == 0 {
            return Ok(());
        }
        core::hint::spin_loop();
    }
    Err("rtl8169 tx timeout")
}
