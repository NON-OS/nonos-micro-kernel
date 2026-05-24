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

use core::arch::asm;

use super::csr::{SIE_SEIE, SIE_SSIE, SIE_STIE, SSTATUS_MXR, SSTATUS_SUM};

pub fn init_cpu() {
    configure_sstatus();
    configure_sie();
}

fn configure_sstatus() {
    let mut sstatus: usize;
    unsafe {
        asm!("csrr {}, sstatus", out(reg) sstatus, options(nostack));
    }
    sstatus |= SSTATUS_SUM | SSTATUS_MXR;
    unsafe {
        asm!("csrw sstatus, {}", in(reg) sstatus, options(nostack));
    }
}

fn configure_sie() {
    let sie: usize = SIE_SSIE | SIE_STIE | SIE_SEIE;
    unsafe {
        asm!("csrw sie, {}", in(reg) sie, options(nostack));
    }
}
