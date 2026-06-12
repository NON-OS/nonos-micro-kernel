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

use super::constants::DS_ROLLBACK;
use super::version::VersionState;

impl VersionState {
    pub fn to_bytes(&self) -> [u8; 48] {
        let mut buf = [0u8; 48];
        buf[0..8].copy_from_slice(&self.kernel_version.to_le_bytes());
        buf[8..16].copy_from_slice(&self.bootloader_version.to_le_bytes());
        buf[16..24].copy_from_slice(&self.minimum_kernel.to_le_bytes());
        buf[24..32].copy_from_slice(&self.minimum_bootloader.to_le_bytes());
        buf[32..40].copy_from_slice(&self.last_boot_timestamp.to_le_bytes());
        buf[40..48].copy_from_slice(&self.boot_count.to_le_bytes());
        buf
    }

    pub fn from_bytes(buf: &[u8; 48]) -> Self {
        Self {
            kernel_version: u64::from_le_bytes([
                buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7],
            ]),
            bootloader_version: u64::from_le_bytes([
                buf[8], buf[9], buf[10], buf[11], buf[12], buf[13], buf[14], buf[15],
            ]),
            minimum_kernel: u64::from_le_bytes([
                buf[16], buf[17], buf[18], buf[19], buf[20], buf[21], buf[22], buf[23],
            ]),
            minimum_bootloader: u64::from_le_bytes([
                buf[24], buf[25], buf[26], buf[27], buf[28], buf[29], buf[30], buf[31],
            ]),
            last_boot_timestamp: u64::from_le_bytes([
                buf[32], buf[33], buf[34], buf[35], buf[36], buf[37], buf[38], buf[39],
            ]),
            boot_count: u64::from_le_bytes([
                buf[40], buf[41], buf[42], buf[43], buf[44], buf[45], buf[46], buf[47],
            ]),
        }
    }

    pub fn compute_hash(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new_derive_key(DS_ROLLBACK);
        hasher.update(&self.to_bytes());
        *hasher.finalize().as_bytes()
    }
}
