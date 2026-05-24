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
use super::{SIE, SIP, SSTATUS};

pub fn set_csr(csr: usize, bits: usize) -> CsrResult<()> {
    match csr {
        SSTATUS => unsafe { asm!("csrs sstatus, {}", in(reg) bits, options(nostack)) },
        SIE => unsafe { asm!("csrs sie, {}", in(reg) bits, options(nostack)) },
        SIP => unsafe { asm!("csrs sip, {}", in(reg) bits, options(nostack)) },
        _ => return Err(CsrError::UnsupportedBitOp(csr)),
    }
    Ok(())
}

pub fn clear_csr(csr: usize, bits: usize) -> CsrResult<()> {
    match csr {
        SSTATUS => unsafe { asm!("csrc sstatus, {}", in(reg) bits, options(nostack)) },
        SIE => unsafe { asm!("csrc sie, {}", in(reg) bits, options(nostack)) },
        SIP => unsafe { asm!("csrc sip, {}", in(reg) bits, options(nostack)) },
        _ => return Err(CsrError::UnsupportedBitOp(csr)),
    }
    Ok(())
}
