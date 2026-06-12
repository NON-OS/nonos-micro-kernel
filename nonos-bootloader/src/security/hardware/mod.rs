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

mod capabilities;
mod cpu;
mod memory;
mod requirements;
mod tpm_detect;
mod verify;

pub use capabilities::{detect_hardware_capabilities, HardwareCapabilities};
pub use cpu::{detect_cpu_security_features, CpuSecurityFeatures};
pub use memory::{detect_memory_protection, MemoryProtection};
pub use requirements::{
    check_minimum_requirements, check_recommended_requirements, RequirementCheck,
};
pub use tpm_detect::{detect_tpm_capabilities, TpmCapabilities};
pub use verify::{verify_platform_security, PlatformVerification};
