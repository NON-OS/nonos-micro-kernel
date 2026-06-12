// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use crate::security::hardware::cpu::CpuSecurityFeatures;
use crate::security::hardware::memory::MemoryProtection;
use crate::security::hardware::tpm_detect::TpmCapabilities;

pub fn calculate_security_score(
    cpu: &CpuSecurityFeatures,
    mem: &MemoryProtection,
    tpm: &TpmCapabilities,
) -> u8 {
    let mut s: u8 = 0;
    if cpu.smep {
        s = s.saturating_add(15);
    }
    if cpu.smap {
        s = s.saturating_add(15);
    }
    if cpu.nx_bit {
        s = s.saturating_add(10);
    }
    if cpu.aes_ni {
        s = s.saturating_add(5);
    }
    if cpu.rdrand {
        s = s.saturating_add(10);
    }
    if mem.dep_enabled {
        s = s.saturating_add(10);
    }
    if mem.aslr_supported {
        s = s.saturating_add(10);
    }
    if tpm.present {
        s = s.saturating_add(15);
    }
    if tpm.version_2_0 {
        s = s.saturating_add(10);
    }
    s
}
