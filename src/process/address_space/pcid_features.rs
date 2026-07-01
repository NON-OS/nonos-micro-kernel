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

pub(super) fn supports_pcid_invalidation() -> bool {
    let cpuid = core::arch::x86_64::__cpuid(1);
    if cpuid.ecx & (1 << 17) == 0 {
        crate::log::log_warning!("[ADDR_SPACE] PCID not supported by CPU");
        return false;
    }
    let ext = core::arch::x86_64::__cpuid_count(7, 0);
    if ext.ebx & (1 << 10) == 0 {
        crate::log::log_warning!("[ADDR_SPACE] INVPCID not supported by CPU");
        return false;
    }
    true
}
