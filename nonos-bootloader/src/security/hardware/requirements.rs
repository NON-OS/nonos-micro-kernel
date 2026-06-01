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

use super::capabilities::HardwareCapabilities;

pub fn check_minimum_requirements(caps: &HardwareCapabilities) -> RequirementCheck {
    RequirementCheck {
        cpu_64bit: true,
        nx_bit: caps.cpu.nx_bit,
        sse2: true,
        min_physical_bits: caps.memory.physical_bits >= 36,
        hardware_rng: caps.cpu.rdrand,
        passed: true,
    }
}

pub fn check_recommended_requirements(caps: &HardwareCapabilities) -> RequirementCheck {
    RequirementCheck {
        cpu_64bit: true,
        nx_bit: caps.cpu.nx_bit,
        sse2: true,
        min_physical_bits: caps.memory.physical_bits >= 48,
        hardware_rng: caps.cpu.rdrand && caps.cpu.rdseed,
        passed: caps.cpu.smep && caps.cpu.smap && caps.tpm.present,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RequirementCheck {
    pub cpu_64bit: bool,
    pub nx_bit: bool,
    pub sse2: bool,
    pub min_physical_bits: bool,
    pub hardware_rng: bool,
    pub passed: bool,
}
