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

pub const PK_LEN: usize = 32;
pub const MAX_KEYS: usize = 16;
pub const MAX_REVOKED: usize = 32;
pub type KeyId = [u8; 32];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyStatus {
    Valid,
    Revoked,
    Unknown,
    VersionTooOld,
    Expired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RevocationReason {
    Unspecified = 0,
    KeyCompromised = 1,
    KeySuperseded = 2,
    AffiliationChanged = 3,
    CessationOfOperation = 4,
}

#[derive(Clone, Copy)]
pub struct RevocationEntry {
    pub key_id: KeyId,
    pub revoked_at: u64,
    pub reason: RevocationReason,
}

impl RevocationEntry {
    pub const fn empty() -> Self {
        Self { key_id: [0u8; 32], revoked_at: 0, reason: RevocationReason::Unspecified }
    }
}
