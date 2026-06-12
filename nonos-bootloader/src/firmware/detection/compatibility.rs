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

use super::hardware::HardwareDevice;
use super::version::FirmwareVersion;

#[derive(Debug, Clone)]
pub struct FirmwareRequirements {
    pub min_version: FirmwareVersion,
    pub max_version: FirmwareVersion,
    pub required_features: u32,
    pub hardware_revision_min: u8,
    pub memory_requirements: u64,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompatibilityResult {
    Compatible,
    IncompatibleVersion,
    IncompatibleHardware,
    InsufficientMemory,
    MissingFeatures,
    UnsupportedDevice,
}
impl Default for FirmwareRequirements {
    fn default() -> Self {
        Self {
            min_version: FirmwareVersion { major: 1, minor: 0, patch: 0, build: 0 },
            max_version: FirmwareVersion { major: 99, minor: 99, patch: 99, build: 9999 },
            required_features: 0,
            hardware_revision_min: 0,
            memory_requirements: 1024 * 1024,
        }
    }
}

pub fn check_firmware_compatibility(
    dev: &HardwareDevice,
    data: &[u8],
    req: &FirmwareRequirements,
) -> CompatibilityResult {
    if dev.revision < req.hardware_revision_min {
        return CompatibilityResult::IncompatibleHardware;
    }
    let ver = extract_version(data);
    if ver.major < req.min_version.major
        || ver.major > req.max_version.major
        || ver.minor < req.min_version.minor
    {
        return CompatibilityResult::IncompatibleVersion;
    }
    let feat = if data.len() >= 20 {
        u32::from_le_bytes([data[16], data[17], data[18], data[19]])
    } else {
        0
    };
    if (feat & req.required_features) != req.required_features {
        return CompatibilityResult::MissingFeatures;
    }
    if data.len() as u64 > req.memory_requirements {
        return CompatibilityResult::InsufficientMemory;
    }
    if dev.vendor_id == 0 || dev.device_id == 0 {
        return CompatibilityResult::UnsupportedDevice;
    }
    CompatibilityResult::Compatible
}

fn extract_version(data: &[u8]) -> FirmwareVersion {
    if data.len() < 16 {
        return FirmwareVersion::default();
    }
    FirmwareVersion {
        major: data[8],
        minor: data[9],
        patch: u16::from_le_bytes([data[10], data[11]]),
        build: u16::from_le_bytes([data[12], data[13]]),
    }
}
