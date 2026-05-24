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

use crate::arch::riscv64::sbi::error::SbiError;

use super::ext::EID_RFENCE;

const FID_REMOTE_SFENCE_VMA_ASID: usize = 2;

pub fn remote_sfence_vma_asid(
    hart_mask: usize,
    hart_mask_base: usize,
    start: usize,
    size: usize,
    asid: usize,
) -> Result<(), SbiError> {
    let error: usize;
    unsafe {
        core::arch::asm!(
            "ecall",
            in("a7") EID_RFENCE,
            in("a6") FID_REMOTE_SFENCE_VMA_ASID,
            inout("a0") hart_mask => error,
            in("a1") hart_mask_base,
            in("a2") start,
            in("a3") size,
            in("a4") asid,
            options(nostack),
        );
    }
    if error == 0 {
        Ok(())
    } else {
        Err(SbiError::from(error as isize))
    }
}
