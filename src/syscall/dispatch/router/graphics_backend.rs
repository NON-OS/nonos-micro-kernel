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

//! Graphics syscall backend. Only the display-dimensions query survives
//! in the kernel; surface create/map/present moved to the MkSurface path
//! served by the surface registry and the compositor capsule. The
//! `surface_span_for_id` helper stays because the live MkSurfacePresent
//! handler (`graphics_present`) shares it.

use crate::syscall::numbers::SyscallNumber;
use crate::syscall::SyscallResult;
use crate::usercopy::write_user_value;

const ENOTSUP: i32 = 95;
const EFAULT: i32 = 14;
const EINVAL: i32 = 22;

pub(super) fn matches(nr: SyscallNumber) -> bool {
    matches!(nr, SyscallNumber::GraphicsDisplayDimensions)
}

pub(super) fn handle(
    nr: SyscallNumber,
    a0: u64,
    a1: u64,
    a2: u64,
    _a3: u64,
    _a4: u64,
    _a5: u64,
) -> SyscallResult {
    match nr {
        SyscallNumber::GraphicsDisplayDimensions => handle_display_dimensions(a0, a1, a2),
        _ => super::super::util::errno(ENOTSUP),
    }
}

fn handle_display_dimensions(display: u64, out_w: u64, out_h: u64) -> SyscallResult {
    if display != 0 || out_w == 0 || out_h == 0 {
        return super::super::util::errno(EINVAL);
    }
    let Some(fb) = crate::kernel_core::init::framebuffer::framebuffer_state() else {
        return super::super::util::errno(ENOTSUP);
    };
    if write_user_value(out_w, &fb.width).is_err() {
        return super::super::util::errno(EFAULT);
    }
    if write_user_value(out_h, &fb.height).is_err() {
        return super::super::util::errno(EFAULT);
    }
    SyscallResult::success_audited(0)
}

/// Resolve a surface id (its base VA) to the length of the mapping that
/// backs it. Shared with `graphics_present`, which serves the live
/// MkSurfacePresent path.
pub(super) fn surface_span_for_id(id: u64) -> Result<usize, i32> {
    if id == 0 {
        return Err(EINVAL);
    }
    let Some(proc) = crate::process::current_process() else {
        return Err(ENOTSUP);
    };
    let mem = proc.memory.lock();
    let Some(vma) = mem.vmas.iter().find(|v| v.start.as_u64() == id) else {
        return Err(EINVAL);
    };
    let len = vma.end.as_u64().saturating_sub(vma.start.as_u64()) as usize;
    if len == 0 {
        return Err(EINVAL);
    }
    Ok(len)
}
