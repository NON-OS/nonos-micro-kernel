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
use super::{
    CYCLE, INSTRET, SATP, SCAUSE, SCOUNTEREN, SEPC, SIE, SIP, SSCRATCH, SSTATUS, STVAL, STVEC, TIME,
};

pub fn read_csr(csr: usize) -> CsrResult<usize> {
    let value: usize;
    match csr {
        SSTATUS => unsafe { asm!("csrr {}, sstatus", out(reg) value, options(nostack)) },
        SIE => unsafe { asm!("csrr {}, sie", out(reg) value, options(nostack)) },
        STVEC => unsafe { asm!("csrr {}, stvec", out(reg) value, options(nostack)) },
        SCOUNTEREN => unsafe { asm!("csrr {}, scounteren", out(reg) value, options(nostack)) },
        SSCRATCH => unsafe { asm!("csrr {}, sscratch", out(reg) value, options(nostack)) },
        SEPC => unsafe { asm!("csrr {}, sepc", out(reg) value, options(nostack)) },
        SCAUSE => unsafe { asm!("csrr {}, scause", out(reg) value, options(nostack)) },
        STVAL => unsafe { asm!("csrr {}, stval", out(reg) value, options(nostack)) },
        SIP => unsafe { asm!("csrr {}, sip", out(reg) value, options(nostack)) },
        SATP => unsafe { asm!("csrr {}, satp", out(reg) value, options(nostack)) },
        CYCLE => unsafe { asm!("csrr {}, cycle", out(reg) value, options(nostack)) },
        TIME => unsafe { asm!("csrr {}, time", out(reg) value, options(nostack)) },
        INSTRET => unsafe { asm!("csrr {}, instret", out(reg) value, options(nostack)) },
        _ => return Err(CsrError::UnsupportedRead(csr)),
    }
    Ok(value)
}

pub fn read_sstatus() -> CsrResult<usize> {
    read_csr(SSTATUS)
}
pub fn read_sepc() -> CsrResult<usize> {
    read_csr(SEPC)
}
pub fn read_scause() -> CsrResult<usize> {
    read_csr(SCAUSE)
}
pub fn read_stval() -> CsrResult<usize> {
    read_csr(STVAL)
}
pub fn read_time() -> CsrResult<u64> {
    read_csr(TIME).map(|v| v as u64)
}
pub fn read_cycle() -> CsrResult<u64> {
    read_csr(CYCLE).map(|v| v as u64)
}
