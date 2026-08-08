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

use crate::kernel_core::surface_registry::RegistryError;
use crate::syscall::dispatch::util::errno;
use crate::syscall::numbers::SyscallNumber;
use crate::syscall::SyscallResult;

use super::surface_handlers as h;

pub(super) const EINVAL: i32 = 22;
pub(super) const EFAULT: i32 = 14;
pub(super) const ENOTSUP: i32 = 95;
pub(super) const ESRCH: i32 = 3;
pub(super) const EPERM: i32 = 1;
pub(super) const ENOMEM: i32 = 12;

pub(super) fn handle(
    nr: SyscallNumber,
    a0: u64,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    _a5: u64,
) -> SyscallResult {
    match nr {
        SyscallNumber::MkSurfaceRegister => h::do_register(a0),
        SyscallNumber::MkSurfaceShare => h::do_share(a0),
        SyscallNumber::MkSurfaceAttach => h::do_attach(a0, a1),
        SyscallNumber::MkSurfaceRelease => h::do_release(a0),
        // a1..a4 carry an optional damage rectangle. All zero means the whole
        // surface, which is what a caller that does not track damage sends, so
        // the older four-argument call keeps working unchanged.
        SyscallNumber::MkSurfacePresent => h::do_present(a0, a1, a2, a3, a4),
        SyscallNumber::MkDisplayVsyncWait => h::do_vsync_wait(a0),
        _ => errno(ENOTSUP),
    }
}

pub(super) fn map_err(e: RegistryError) -> i32 {
    match e {
        RegistryError::InvalidArg | RegistryError::BadHandle | RegistryError::NotFound => EINVAL,
        RegistryError::NotOwner => EPERM,
        RegistryError::OutOfSlots => ENOMEM,
        RegistryError::MapFailed | RegistryError::NoProc => ENOTSUP,
    }
}
