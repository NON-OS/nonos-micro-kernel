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

use super::error::ParseError;
use crate::image_format::footer::{ImageFooter, FOOTER_SIZE};

pub fn parse_footer_bytes(bytes: &[u8]) -> Result<ImageFooter, ParseError> {
    if bytes.len() < FOOTER_SIZE {
        return Err(ParseError::ImageTooSmall);
    }

    let mut magic = [0u8; 8];
    magic.copy_from_slice(&bytes[0..8]);

    let version = u16::from_le_bytes([bytes[8], bytes[9]]);
    let flags = u16::from_le_bytes([bytes[10], bytes[11]]);
    let hash_algorithm = bytes[12];
    let signature_algorithm = bytes[13];
    let reserved0 = u16::from_le_bytes([bytes[14], bytes[15]]);

    let total_image_size = u64::from_le_bytes([
        bytes[16], bytes[17], bytes[18], bytes[19], bytes[20], bytes[21], bytes[22], bytes[23],
    ]);

    let kernel_offset = u32::from_le_bytes([bytes[24], bytes[25], bytes[26], bytes[27]]);
    let kernel_size = u32::from_le_bytes([bytes[28], bytes[29], bytes[30], bytes[31]]);
    let signature_offset = u32::from_le_bytes([bytes[32], bytes[33], bytes[34], bytes[35]]);
    let signature_size = u32::from_le_bytes([bytes[36], bytes[37], bytes[38], bytes[39]]);
    let proof_offset = u32::from_le_bytes([bytes[40], bytes[41], bytes[42], bytes[43]]);
    let proof_size = u32::from_le_bytes([bytes[44], bytes[45], bytes[46], bytes[47]]);
    let image_version = u32::from_le_bytes([bytes[48], bytes[49], bytes[50], bytes[51]]);

    let mut reserved1 = [0u8; 4];
    reserved1.copy_from_slice(&bytes[52..56]);

    Ok(ImageFooter {
        magic,
        version,
        flags,
        hash_algorithm,
        signature_algorithm,
        reserved0,
        total_image_size,
        kernel_offset,
        kernel_size,
        signature_offset,
        signature_size,
        proof_offset,
        proof_size,
        image_version,
        reserved1,
    })
}
