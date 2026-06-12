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

use super::types::UserEntry;
use crate::arch::riscv64::cpu::csr::{SSTATUS_SPIE, SSTATUS_SPP};

extern "C" {
    fn riscv64_enter_user(ctx: *const UserEntry) -> !;
}

pub const SSTATUS_USER_INITIAL: u64 = SSTATUS_SPIE as u64;

const USER_VA_MAX_SV39: u64 = (1u64 << 38) - 1;

#[derive(Debug, Clone, Copy)]
pub enum EnterError {
    NonUserEntry,
    NonUserStack,
    NoKernelStack,
    SstatusWouldStayInSMode,
}

pub unsafe fn enter_user(ctx: &UserEntry) -> Result<core::convert::Infallible, EnterError> {
    if ctx.entry == 0 || ctx.entry > USER_VA_MAX_SV39 {
        return Err(EnterError::NonUserEntry);
    }
    if ctx.user_sp == 0 || ctx.user_sp > USER_VA_MAX_SV39 {
        return Err(EnterError::NonUserStack);
    }
    if ctx.kernel_sp == 0 {
        return Err(EnterError::NoKernelStack);
    }
    if (ctx.sstatus & SSTATUS_SPP as u64) != 0 {
        return Err(EnterError::SstatusWouldStayInSMode);
    }
    unsafe { riscv64_enter_user(ctx as *const _) }
}
