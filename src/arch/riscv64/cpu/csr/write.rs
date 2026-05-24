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

use super::{CsrError, CsrResult};
use super::{SATP, SCAUSE, SCOUNTEREN, SEPC, SIE, SIP, SSCRATCH, SSTATUS, STVAL, STVEC};

pub fn write_csr(csr: usize, value: usize) -> CsrResult<()> {
    match csr {
        SSTATUS => unsafe { asm!("csrw sstatus, {}", in(reg) value, options(nostack)) },
        SIE => unsafe { asm!("csrw sie, {}", in(reg) value, options(nostack)) },
        STVEC => unsafe { asm!("csrw stvec, {}", in(reg) value, options(nostack)) },
        SCOUNTEREN => unsafe { asm!("csrw scounteren, {}", in(reg) value, options(nostack)) },
        SSCRATCH => unsafe { asm!("csrw sscratch, {}", in(reg) value, options(nostack)) },
        SEPC => unsafe { asm!("csrw sepc, {}", in(reg) value, options(nostack)) },
        SCAUSE => unsafe { asm!("csrw scause, {}", in(reg) value, options(nostack)) },
        STVAL => unsafe { asm!("csrw stval, {}", in(reg) value, options(nostack)) },
        SIP => unsafe { asm!("csrw sip, {}", in(reg) value, options(nostack)) },
        SATP => unsafe { asm!("csrw satp, {}", in(reg) value, options(nostack)) },
        _ => return Err(CsrError::UnsupportedWrite(csr)),
    }
    Ok(())
}
