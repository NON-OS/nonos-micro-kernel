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

use super::machine::get_machine_id;
use super::nonce::get_boot_nonce;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ZkPublicInputs {
    pub kernel_hash: [u8; 32],
    pub boot_nonce: [u8; 32],
    pub timestamp: u64,
    pub machine_id: [u8; 32],
}

impl ZkPublicInputs {
    pub fn to_bytes(&self) -> [u8; 104] {
        let mut buf = [0u8; 104];
        buf[0..32].copy_from_slice(&self.kernel_hash);
        buf[32..64].copy_from_slice(&self.boot_nonce);
        buf[64..72].copy_from_slice(&self.timestamp.to_le_bytes());
        buf[72..104].copy_from_slice(&self.machine_id);
        buf
    }

    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        if data.len() < 104 {
            return None;
        }
        let mut inputs = Self {
            kernel_hash: [0u8; 32],
            boot_nonce: [0u8; 32],
            timestamp: 0,
            machine_id: [0u8; 32],
        };
        inputs.kernel_hash.copy_from_slice(&data[0..32]);
        inputs.boot_nonce.copy_from_slice(&data[32..64]);
        inputs.timestamp = u64::from_le_bytes(data[64..72].try_into().ok()?);
        inputs.machine_id.copy_from_slice(&data[72..104]);
        Some(inputs)
    }
}

pub fn build_public_inputs(
    kernel_hash: [u8; 32],
    timestamp: u64,
) -> Result<ZkPublicInputs, &'static str> {
    Ok(ZkPublicInputs {
        kernel_hash,
        boot_nonce: get_boot_nonce()?,
        timestamp,
        machine_id: get_machine_id()?,
    })
}
