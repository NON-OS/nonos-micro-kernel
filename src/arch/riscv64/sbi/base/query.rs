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

use crate::arch::riscv64::sbi::SbiError;

use super::call::sbi_call;
use super::constants::*;

fn value(fid: usize, a0: usize) -> Result<usize, SbiError> {
    let ret = sbi_call(EID_BASE, fid, a0, 0, 0);
    if ret.error != 0 {
        Err(SbiError::from(ret.error))
    } else {
        Ok(ret.value)
    }
}

pub fn sbi_version() -> Result<(u32, u32), SbiError> {
    let raw = value(FID_GET_SPEC_VERSION, 0)?;
    Ok((((raw >> 24) & 0x7F) as u32, (raw & 0xFF_FFFF) as u32))
}

pub fn impl_id() -> Result<usize, SbiError> { value(FID_GET_IMPL_ID, 0) }
pub fn impl_version() -> Result<usize, SbiError> { value(FID_GET_IMPL_VERSION, 0) }
pub fn mvendorid() -> Result<usize, SbiError> { value(FID_GET_MVENDORID, 0) }
pub fn marchid() -> Result<usize, SbiError> { value(FID_GET_MARCHID, 0) }
pub fn mimpid() -> Result<usize, SbiError> { value(FID_GET_MIMPID, 0) }

pub fn probe_extension_base(eid: usize) -> Result<bool, SbiError> {
    value(FID_PROBE_EXTENSION, eid).map(|v| v != 0)
}
