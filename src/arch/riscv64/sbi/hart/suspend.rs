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

use super::control::hart_suspend;

pub const SUSPEND_DEFAULT_RETENTIVE: u32 = 0x0000_0000;
pub const SUSPEND_DEFAULT_NON_RETENTIVE: u32 = 0x8000_0000;

pub fn suspend_retentive() -> Result<(), SbiError> {
    hart_suspend(SUSPEND_DEFAULT_RETENTIVE, 0, 0)
}

pub fn suspend_non_retentive(resume_addr: u64, opaque: u64) -> Result<(), SbiError> {
    hart_suspend(SUSPEND_DEFAULT_NON_RETENTIVE, resume_addr, opaque)
}
