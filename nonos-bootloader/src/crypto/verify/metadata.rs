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

use crate::crypto::keys::KeyId;

#[derive(Debug, Clone)]
pub struct CapsuleMetadata {
    pub offset_sig: usize,
    pub len_sig: usize,
    pub offset_payload: usize,
    pub len_payload: usize,
    pub signer_keyid: Option<KeyId>,
    pub payload_hash: [u8; 32],
    pub header_version: u32,
    pub header_timestamp: u64,
}

impl Default for CapsuleMetadata {
    fn default() -> Self {
        Self {
            offset_sig: 0,
            len_sig: 0,
            offset_payload: 0,
            len_payload: 0,
            signer_keyid: None,
            payload_hash: [0u8; 32],
            header_version: 0,
            header_timestamp: 0,
        }
    }
}
