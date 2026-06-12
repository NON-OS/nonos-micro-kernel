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
pub enum TpmError {
    NotPresent,
    NotReady,
    Timeout,
    InvalidResponse,
    NvIndexNotFound,
    NvAccessDenied,
    NvSizeMismatch,
    CommandFailed(u32),
}

impl core::fmt::Display for TpmError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotPresent => write!(f, "TPM not present"),
            Self::NotReady => write!(f, "TPM not ready"),
            Self::Timeout => write!(f, "TPM timeout"),
            Self::InvalidResponse => write!(f, "invalid TPM response"),
            Self::NvIndexNotFound => write!(f, "NV index not found"),
            Self::NvAccessDenied => write!(f, "NV access denied"),
            Self::NvSizeMismatch => write!(f, "NV size mismatch"),
            Self::CommandFailed(rc) => write!(f, "TPM command failed: 0x{:X}", rc),
        }
    }
}

#[derive(Clone, Copy)]
pub struct NvIndex {
    index: u32,
}
impl NvIndex {
    pub const fn new(index: u32) -> Self {
        Self { index }
    }
    pub fn raw(&self) -> u32 {
        self.index
    }
}
