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

use crate::syscall::numbers::SyscallNumber;
use crate::syscall::SyscallResult;

use super::errno::E_NOSYS;
use super::policy_push::policy_push;
use super::reboot::reboot;
use super::shutdown::shutdown;

pub(in crate::syscall::dispatch::router) fn matches(nr: SyscallNumber) -> bool {
    matches!(
        nr,
        SyscallNumber::AdminReboot
            | SyscallNumber::AdminShutdown
            | SyscallNumber::AdminPolicyPush
    )
}

pub(in crate::syscall::dispatch::router) fn handle(
    nr: SyscallNumber,
    a0: u64,
    a1: u64,
    a2: u64,
    a3: u64,
    _a4: u64,
    _a5: u64,
) -> SyscallResult {
    let value = match nr {
        SyscallNumber::AdminReboot => reboot(),
        SyscallNumber::AdminShutdown => shutdown(),
        SyscallNumber::AdminPolicyPush => policy_push(a0, a1, a2, a3),
        _ => E_NOSYS,
    };
    SyscallResult { value, capability_consumed: false, audit_required: true }
}
