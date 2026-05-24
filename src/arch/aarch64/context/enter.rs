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


extern "C" {
    fn aarch64_enter_user(ctx: *const UserEntry) -> !;
}
















pub const SPSR_EL0T_INITIAL: u64 = 0;



const USER_VA_MAX: u64 = 0x0000_FFFF_FFFF_FFFF;

#[derive(Debug, Clone, Copy)]
pub enum EnterError {
    NonUserEntry,
    NonUserStack,
    NoKernelStack,
}











pub unsafe fn enter_user(ctx: &UserEntry) -> Result<core::convert::Infallible, EnterError> {
    if ctx.entry == 0 || ctx.entry > USER_VA_MAX {
        return Err(EnterError::NonUserEntry);
    }
    if ctx.user_sp == 0 || ctx.user_sp > USER_VA_MAX {
        return Err(EnterError::NonUserStack);
    }
    if ctx.kernel_sp == 0 {
        return Err(EnterError::NoKernelStack);
    }
    unsafe { aarch64_enter_user(ctx as *const _) }
}
