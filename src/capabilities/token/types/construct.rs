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

use super::CapabilityToken;
use crate::capabilities::types::Capability;
use alloc::vec::Vec;

impl CapabilityToken {
    pub fn empty() -> Self {
        Self {
            owner_module: 0,
            permissions: Vec::new(),
            expires_at_ms: Some(0),
            nonce: 0,
            signature: [0u8; 64],
            token_id: 0,
            subject_capsule_id: 0,
            subject_asid: 0,
            subject_measurement: [0u8; 32],
            boot_session_nonce: [0u8; 16],
            revocation_epoch: 0,
            delegation_depth: 0,
        }
    }
    /// Empty-token constructor. Binds no boot session nonce so it
    /// never accidentally lends an empty token live boot authority.
    pub fn with_caps(caps: &[Capability]) -> Self {
        Self {
            owner_module: 0,
            permissions: caps.to_vec(),
            expires_at_ms: None,
            nonce: 0,
            signature: [0u8; 64],
            token_id: 0,
            subject_capsule_id: 0,
            subject_asid: 0,
            subject_measurement: [0u8; 32],
            boot_session_nonce: [0u8; 16],
            revocation_epoch: 0,
            delegation_depth: 0,
        }
    }
    pub fn system() -> Self {
        use crate::capabilities::types::Capability::*;
        Self::with_caps(&[
            CoreExec,
            IO,
            FileSystem,
            Memory,
            Network,
            IPC,
            Crypto,
            Hardware,
            Debug,
            Admin,
            RegisterService,
        ])
    }
}
