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

use crate::arch::x86_64::context::rflags;
use crate::process::userspace::{UserContext, USER_CS, USER_DS};

const USER_SPACE_MAX: u64 = 0x0000_7FFF_FFFF_FFFF;

pub(super) fn sanitize_user_context(ctx: &mut UserContext) -> bool {
    if ctx.cs != USER_CS as u64 || ctx.ss != USER_DS as u64 {
        return false;
    }
    if ctx.rip > USER_SPACE_MAX || ctx.rsp > USER_SPACE_MAX || ctx.rsp == 0 {
        return false;
    }
    ctx.rflags = rflags::sanitize_user(ctx.rflags);
    true
}
