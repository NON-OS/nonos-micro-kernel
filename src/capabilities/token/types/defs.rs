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

extern crate alloc;

use crate::capabilities::types::Capability;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct CapabilityToken {
    pub owner_module: u64,
    pub permissions: Vec<Capability>,
    pub expires_at_ms: Option<u64>,
    pub nonce: u64,
    pub signature: [u8; 64],
    // Authority material added in Phase 2 of the capability rewrite.
    // Populated on every mint; covered by the MAC. Enforcement lands
    // in a later step — this commit only carries the data.
    pub token_id: u64,
    pub subject_capsule_id: u32,
    pub subject_asid: u32,
    pub subject_measurement: [u8; 32],
    pub boot_session_nonce: [u8; 16],
    pub revocation_epoch: u64,
    pub delegation_depth: u8,
}
