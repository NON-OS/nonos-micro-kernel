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

use crate::security::attestation::pcr::{PcrValue, MAX_PCRS};

pub struct AttestationState {
    pub(super) pcrs: [PcrValue; MAX_PCRS],
    pub(super) kernel_hash: [u8; 32],
    pub(super) bootloader_hash: [u8; 32],
    pub(super) zk_verified: bool,
    pub(super) sig_verified: bool,
    pub(super) program_hash: [u8; 32],
    pub(super) capsule_commitment: [u8; 32],
    pub(super) initialized: bool,
}

impl AttestationState {
    pub const fn new() -> Self {
        Self {
            pcrs: [
                PcrValue::empty(0),
                PcrValue::empty(1),
                PcrValue::empty(2),
                PcrValue::empty(3),
                PcrValue::empty(4),
                PcrValue::empty(5),
                PcrValue::empty(6),
                PcrValue::empty(7),
                PcrValue::empty(8),
                PcrValue::empty(9),
                PcrValue::empty(10),
                PcrValue::empty(11),
                PcrValue::empty(12),
                PcrValue::empty(13),
                PcrValue::empty(14),
                PcrValue::empty(15),
                PcrValue::empty(16),
                PcrValue::empty(17),
                PcrValue::empty(18),
                PcrValue::empty(19),
                PcrValue::empty(20),
                PcrValue::empty(21),
                PcrValue::empty(22),
                PcrValue::empty(23),
            ],
            kernel_hash: [0u8; 32],
            bootloader_hash: [0u8; 32],
            zk_verified: false,
            sig_verified: false,
            program_hash: [0u8; 32],
            capsule_commitment: [0u8; 32],
            initialized: false,
        }
    }

    pub fn init(&mut self) {
        self.initialized = true;
    }
}
