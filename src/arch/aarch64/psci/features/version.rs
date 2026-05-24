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

use super::super::function::PSCI_VERSION;
use super::super::raw::psci_call0;

#[derive(Debug, Clone, Copy)]
pub struct PsciVersion {
    pub major: u16,
    pub minor: u16,
}

impl PsciVersion {
    pub fn from_raw(raw: u32) -> Self {
        Self { major: (raw >> 16) as u16, minor: (raw & 0xFFFF) as u16 }
    }

    pub fn is_v1(&self) -> bool {
        self.major >= 1
    }

    pub fn supports_features(&self) -> bool {
        self.major >= 1
    }
}

impl core::fmt::Display for PsciVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

pub fn psci_version() -> PsciVersion {
    PsciVersion::from_raw(psci_call0(PSCI_VERSION) as u32)
}
