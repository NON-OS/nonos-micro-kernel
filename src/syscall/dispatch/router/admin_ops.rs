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

use crate::arch::x86_64::acpi::power_reboot;
use crate::syscall::numbers::SyscallNumber;
use crate::syscall::SyscallResult;

pub(super) fn matches(nr: SyscallNumber) -> bool {
    matches!(nr, SyscallNumber::AdminReboot | SyscallNumber::AdminShutdown)
}

pub(super) fn handle(nr: SyscallNumber, _a0: u64, _a1: u64, _a2: u64, _a3: u64, _a4: u64, _a5: u64) -> SyscallResult {
    let value = match nr {
        SyscallNumber::AdminReboot => {
            let _ = power_reboot::reboot();
            0
        }
        SyscallNumber::AdminShutdown => -95,
        _ => -38,
    };
    SyscallResult { value, capability_consumed: false, audit_required: true }
}
