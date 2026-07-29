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

use crate::arch::power::PowerOff;
use crate::security::zerostate::terminate;

/// Reset the machine, wiping on the way.
///
/// A warm reset leaves DRAM powered and its rows intact, so this path needs
/// the wipe at least as much as the power-off one does. It used to call ACPI
/// straight out of the syscall router, which both skipped the wipe and only
/// built on x86_64.
pub(super) fn reboot() -> ! {
    terminate(PowerOff::Reboot)
}
