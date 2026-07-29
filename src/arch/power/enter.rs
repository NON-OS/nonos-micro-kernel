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

use super::kind::PowerOff;

/// Hand the machine to the firmware and never come back.
///
/// Divergent on purpose. A power routine that can return leaves the caller a
/// path that runs on after the machine was supposed to be gone, and the whole
/// point of routing every exit through one place is that no such path exists.
/// Where the firmware call can fail, the fallback is to park the CPU rather
/// than to unwind.
pub(crate) fn enter(off: PowerOff) -> ! {
    #[cfg(target_arch = "x86_64")]
    {
        match off {
            PowerOff::Shutdown => {
                let _ = crate::arch::x86_64::acpi::power::shutdown();
            }
            PowerOff::Reboot => {
                let _ = crate::arch::x86_64::acpi::power_reboot::reboot();
            }
        }
        crate::arch::halt_loop()
    }
    #[cfg(target_arch = "aarch64")]
    {
        match off {
            PowerOff::Shutdown => crate::arch::aarch64::psci::system_off(),
            PowerOff::Reboot => crate::arch::aarch64::psci::system_reset(),
        }
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        let _ = off;
        crate::arch::halt_loop()
    }
}
