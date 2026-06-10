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

#[derive(Debug, Clone, Copy)]
pub struct HardwareCapabilities {
    pub cpu: CpuSecurityFeatures,
    pub memory: MemoryProtection,
    pub tpm: TpmCapabilities,
    pub security_score: u8,
}

impl HardwareCapabilities {
    pub fn is_production_ready(&self) -> bool {
        self.cpu.smep
            && self.cpu.smap
            && self.cpu.nx_bit
            && self.memory.dep_enabled
            && self.security_score >= 70
    }

    pub fn has_measured_boot_support(&self) -> bool {
        self.tpm.present && self.tpm.version_2_0 && self.cpu.tpm_support
    }

    pub fn has_memory_encryption(&self) -> bool {
        self.memory.sme_available || self.memory.sev_available || self.memory.tme_available
    }
}
