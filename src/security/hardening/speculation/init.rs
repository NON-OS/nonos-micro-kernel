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

/// Detect what this CPU is vulnerable to and turn on what it supports.
pub fn init() -> Result<(), &'static str> {
    #[cfg(target_arch = "x86_64")]
    return super::super::spectre_mitigations::init();

    #[cfg(target_arch = "aarch64")]
    {
        // The aarch64 boot path already ran this before kernel-core started:
        // SSBS has to be on before the first EL0 entry, which happens well
        // ahead of the security subsystem coming up. Calling it again is
        // harmless and keeps the ordering honest if that ever changes.
        crate::arch::aarch64::security::init_spectre_mitigations();
        Ok(())
    }

    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    Ok(())
}
