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

use crate::footer::{create_image_footer, FOOTER_SIZE};
use crate::kernel::SignedKernel;

pub struct AttestedImage {
    pub data: Vec<u8>,
    pub kernel_size: u32,
    pub signature_size: u32,
    pub proof_size: u32,
}

pub fn assemble_attested_image(kernel: &SignedKernel, zk_block: Vec<u8>) -> AttestedImage {
    let kernel_size = kernel.kernel_bytes.len() as u32;
    let signature_size = kernel.signature.len() as u32;
    let proof_size = zk_block.len() as u32;

    let total_size = kernel.kernel_bytes.len() + kernel.signature.len() + zk_block.len() + FOOTER_SIZE;
    let footer = create_image_footer(
        kernel_size,
        signature_size,
        kernel.signature_algorithm,
        proof_size,
        total_size as u64,
        kernel.rollback_index,
    );

    let mut data = Vec::with_capacity(total_size);
    data.extend_from_slice(&kernel.kernel_bytes);
    data.extend_from_slice(&kernel.signature);
    data.extend_from_slice(&zk_block);
    data.extend_from_slice(&footer);

    AttestedImage { data, kernel_size, signature_size, proof_size }
}
