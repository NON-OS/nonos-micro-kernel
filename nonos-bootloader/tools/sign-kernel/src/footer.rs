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

use crate::constants::*;

pub fn create_image_footer(
    kernel_size: u32,
    signature_size: u32,
    signature_algorithm: u8,
    total_image_size: u64,
    rollback_index: u32,
) -> [u8; FOOTER_SIZE] {
    let mut footer = [0u8; FOOTER_SIZE];
    footer[0..8].copy_from_slice(&FOOTER_MAGIC);
    footer[8..10].copy_from_slice(&FOOTER_VERSION.to_le_bytes());
    footer[10..12].copy_from_slice(&0u16.to_le_bytes());
    footer[12] = HASH_ALG_BLAKE3;
    footer[13] = signature_algorithm;
    footer[16..24].copy_from_slice(&total_image_size.to_le_bytes());
    footer[28..32].copy_from_slice(&kernel_size.to_le_bytes());
    footer[32..36].copy_from_slice(&kernel_size.to_le_bytes());
    footer[36..40].copy_from_slice(&signature_size.to_le_bytes());
    footer[48..52].copy_from_slice(&1u32.to_le_bytes());
    footer[56..60].copy_from_slice(&rollback_index.to_le_bytes());
    footer
}
