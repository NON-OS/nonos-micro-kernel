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

use crate::arch::riscv64::asm::trap_entry_addr;
use crate::arch::riscv64::cpu::csr::{write_csr, STVEC};

const STVEC_MODE_DIRECT: usize = 0;

pub fn install_stvec() {
    let base = trap_entry_addr();
    if base & 0x3 != 0 {
        crate::arch::riscv64::cpu::halt();
    }
    if write_csr(STVEC, base | STVEC_MODE_DIRECT).is_err() {
        crate::arch::riscv64::cpu::halt();
    }
}
