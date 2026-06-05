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

//! Validation report a marketplace operator publishes alongside
//! each release. The capsule consumes this; it does not produce one.

extern crate alloc;

use alloc::string::String;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ValidationStatus {
    Unknown = 0,
    Pending = 1,
    Validated = 2,
    Rejected = 3,
}

impl ValidationStatus {
    pub fn from_u8(b: u8) -> Self {
        match b {
            1 => Self::Pending,
            2 => Self::Validated,
            3 => Self::Rejected,
            _ => Self::Unknown,
        }
    }

    pub fn is_validated(self) -> bool {
        matches!(self, Self::Validated)
    }
}

#[derive(Clone)]
pub struct ValidationReport {
    /// Marketplace operator's status verdict.
    pub status: ValidationStatus,
    /// Free-form note explaining the verdict; capped at
    /// `MAX_DESCRIPTION` bytes.
    pub note: String,
    /// Operator identifier ("nonos.marketplace.v1" today).
    pub validator_id: String,
    /// Unix-millis timestamp of the verdict; 0 when unset.
    pub validated_at_ms: u64,
}
