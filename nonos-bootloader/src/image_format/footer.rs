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

pub const FOOTER_MAGIC: [u8; 8] = *b"NONOSIMG";
pub const FOOTER_VERSION: u16 = 1;
pub const FOOTER_SIZE: usize = 64;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ImageFooter {
    pub magic: [u8; 8],
    pub version: u16,
    pub flags: u16,
    pub hash_algorithm: u8,
    pub signature_algorithm: u8,
    pub reserved0: u16,
    pub total_image_size: u64,
    pub kernel_offset: u32,
    pub kernel_size: u32,
    pub signature_offset: u32,
    pub signature_size: u32,
    pub proof_offset: u32,
    pub proof_size: u32,
    pub image_version: u32,
    pub reserved1: [u8; 4],
    pub rollback_index: u32,
    pub reserved2: [u8; 4],
}

impl ImageFooter {
    pub const fn is_valid_magic(&self) -> bool {
        self.magic[0] == FOOTER_MAGIC[0]
            && self.magic[1] == FOOTER_MAGIC[1]
            && self.magic[2] == FOOTER_MAGIC[2]
            && self.magic[3] == FOOTER_MAGIC[3]
            && self.magic[4] == FOOTER_MAGIC[4]
            && self.magic[5] == FOOTER_MAGIC[5]
            && self.magic[6] == FOOTER_MAGIC[6]
            && self.magic[7] == FOOTER_MAGIC[7]
    }

    pub const fn is_supported_version(&self) -> bool {
        self.version == FOOTER_VERSION
    }

    pub fn kernel_end(&self) -> Option<u64> {
        (self.kernel_offset as u64).checked_add(self.kernel_size as u64)
    }

    pub fn signature_end(&self) -> Option<u64> {
        (self.signature_offset as u64).checked_add(self.signature_size as u64)
    }

    pub fn proof_end(&self) -> Option<u64> {
        if self.proof_size == 0 {
            return Some(self.proof_offset as u64);
        }
        (self.proof_offset as u64).checked_add(self.proof_size as u64)
    }

    pub fn has_zk_proof(&self) -> bool {
        self.proof_size > 0 && (self.flags & (super::types::flags::HAS_ZK_PROOF as u16)) != 0
    }
}
