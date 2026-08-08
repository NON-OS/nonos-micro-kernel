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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SmmError {
    NotInitialized,
    AlreadyInitialized,
    UnsupportedCpu,
    RegionDetectionFailed { reason: &'static str },
    HandlerEnumerationFailed { reason: &'static str },
    ProtectionFailed { reason: &'static str },
    IntegrityCheckFailed { handler: u64 },
    SmramNotLocked,
    SmramOpen,
    InvalidHandler { address: u64 },
    MsrAccessFailed { msr: u32 },
    PciAccessFailed,
    AcpiPmBaseNotFound,
}

impl SmmError {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::NotInitialized => "SMM subsystem not initialized",
            Self::AlreadyInitialized => "SMM subsystem already initialized",
            Self::UnsupportedCpu => "CPU vendor not supported for SMM protection",
            Self::RegionDetectionFailed { .. } => "Failed to detect SMM regions",
            Self::HandlerEnumerationFailed { .. } => "Failed to enumerate SMM handlers",
            Self::ProtectionFailed { .. } => "Failed to enable SMM protection",
            Self::IntegrityCheckFailed { .. } => "SMM integrity check failed",
            Self::SmramNotLocked => "SMRAM is not locked",
            Self::SmramOpen => "SMRAM is open to CPU access",
            Self::InvalidHandler { .. } => "Invalid SMM handler address",
            Self::MsrAccessFailed { .. } => "MSR access failed",
            Self::PciAccessFailed => "PCI configuration access failed",
            Self::AcpiPmBaseNotFound => "ACPI PM base address not found",
        }
    }
}

pub type SmmResult<T> = Result<T, SmmError>;
