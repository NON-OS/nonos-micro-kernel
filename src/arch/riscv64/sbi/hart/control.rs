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
use crate::arch::riscv64::sbi::SbiError;

use super::constants::*;
use super::status::HartStatus;

fn unit(fid: usize, a0: usize, a1: usize, a2: usize) -> Result<(), SbiError> {
    let ret = sbi_call(EID_HSM, fid, a0, a1, a2);
    if ret.error != 0 {
        Err(SbiError::from(ret.error))
    } else {
        Ok(())
    }
}

pub fn hart_start(hartid: u64, start_addr: u64, opaque: u64) -> Result<(), SbiError> {
    unit(FID_HART_START, hartid as usize, start_addr as usize, opaque as usize)
}

pub fn hart_stop() -> Result<(), SbiError> {
    unit(FID_HART_STOP, 0, 0, 0)
}

pub fn hart_get_status(hartid: u64) -> Result<HartStatus, SbiError> {
    let ret = sbi_call(EID_HSM, FID_HART_GET_STATUS, hartid as usize, 0, 0);
    if ret.error != 0 {
        Err(SbiError::from(ret.error))
    } else {
        Ok(HartStatus::from(ret.value))
    }
}

pub fn hart_suspend(suspend_type: u32, resume_addr: u64, opaque: u64) -> Result<(), SbiError> {
    unit(FID_HART_SUSPEND, suspend_type as usize, resume_addr as usize, opaque as usize)
}
