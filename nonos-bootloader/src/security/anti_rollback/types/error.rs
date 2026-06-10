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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RollbackError {
    KernelVersionTooOld { kernel: u64, minimum: u64 },
    BootloaderVersionTooOld { current: u64, minimum: u64 },
    NvramReadFailed,
    NvramWriteFailed,
    TpmNotAvailable,
    InvalidVersion,
}

impl RollbackError {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::KernelVersionTooOld { .. } => "kernel version too old",
            Self::BootloaderVersionTooOld { .. } => "bootloader version too old",
            Self::NvramReadFailed => "NVRAM read failed",
            Self::NvramWriteFailed => "NVRAM write failed",
            Self::TpmNotAvailable => "TPM not available",
            Self::InvalidVersion => "invalid version",
        }
    }
}

impl core::fmt::Display for RollbackError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::KernelVersionTooOld { kernel, minimum } => {
                write!(f, "kernel {} < minimum {}", kernel, minimum)
            }
            Self::BootloaderVersionTooOld { current, minimum } => {
                write!(f, "bootloader {} < minimum {}", current, minimum)
            }
            Self::NvramReadFailed => write!(f, "NVRAM read failed"),
            Self::NvramWriteFailed => write!(f, "NVRAM write failed"),
            Self::TpmNotAvailable => write!(f, "TPM not available"),
            Self::InvalidVersion => write!(f, "invalid version"),
        }
    }
}
