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

use crate::arch::riscv64::sbi::base::sbi_call;
use crate::arch::riscv64::sbi::error::SbiError;

use super::ext::EID_RFENCE;

const FID_REMOTE_FENCE_I: usize = 0;

pub fn remote_fence_i(hart_mask: usize, hart_mask_base: usize) -> Result<(), SbiError> {
    let ret = sbi_call(EID_RFENCE, FID_REMOTE_FENCE_I, hart_mask, hart_mask_base, 0);
    if ret.error == 0 {
        Ok(())
    } else {
        Err(SbiError::from(ret.error))
    }
}
