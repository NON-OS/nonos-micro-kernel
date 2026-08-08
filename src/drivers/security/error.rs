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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverError {
    InvalidMmioRegion,
    MmioAccessDenied,
    InvalidDmaBuffer,
    InvalidPrpList,
    InvalidPciAccess,
    ConfigWriteDenied,
    LbaOutOfRange,
    RateLimitExceeded,
    DeviceNotReady,
    CommandTimeout,
    IntegrityCheckFailed,
}

impl DriverError {
    pub fn code(&self) -> u32 {
        match self {
            DriverError::InvalidMmioRegion => 0x1001,
            DriverError::MmioAccessDenied => 0x1002,
            DriverError::InvalidDmaBuffer => 0x2001,
            DriverError::InvalidPrpList => 0x2002,
            DriverError::InvalidPciAccess => 0x3001,
            DriverError::ConfigWriteDenied => 0x3002,
            DriverError::LbaOutOfRange => 0x4001,
            DriverError::RateLimitExceeded => 0x5001,
            DriverError::DeviceNotReady => 0x6001,
            DriverError::CommandTimeout => 0x6002,
            DriverError::IntegrityCheckFailed => 0x7001,
        }
    }

    pub fn is_security_critical(&self) -> bool {
        matches!(
            self,
            DriverError::InvalidMmioRegion
                | DriverError::MmioAccessDenied
                | DriverError::InvalidDmaBuffer
                | DriverError::InvalidPrpList
                | DriverError::ConfigWriteDenied
                | DriverError::IntegrityCheckFailed
        )
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            DriverError::InvalidMmioRegion => "Invalid MMIO region",
            DriverError::MmioAccessDenied => "MMIO access denied",
            DriverError::InvalidDmaBuffer => "Invalid DMA buffer",
            DriverError::InvalidPrpList => "Invalid PRP list",
            DriverError::InvalidPciAccess => "Invalid PCI access",
            DriverError::ConfigWriteDenied => "PCI config write denied",
            DriverError::LbaOutOfRange => "LBA out of range",
            DriverError::RateLimitExceeded => "Rate limit exceeded",
            DriverError::DeviceNotReady => "Device not ready",
            DriverError::CommandTimeout => "Command timeout",
            DriverError::IntegrityCheckFailed => "Integrity check failed",
        }
    }
}

impl core::fmt::Display for DriverError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
