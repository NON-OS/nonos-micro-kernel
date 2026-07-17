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

//! Assemble an attested image the way the build does, and parse its footer the
//! way the bootloader does. The parse is defensive: any out-of-range offset
//! returns None rather than indexing past the buffer.

use embed_zk_proof::{assemble_attested_image, SignedKernel, FOOTER_SIZE};

/// Assemble an attested image, for tools that need a subject to inspect or attack.
pub fn assemble_image(kernel_bytes: &[u8], trailer: Vec<u8>) -> Vec<u8> {
    let kernel = SignedKernel {
        raw_bytes: Vec::new(),
        kernel_bytes: kernel_bytes.to_vec(),
        signature: vec![0u8; 64],
        signature_algorithm: 1,
        rollback_index: 1,
    };
    assemble_attested_image(&kernel, trailer).data
}

/// The bootloader's footer parse: recover the kernel region and the proof region.
/// Returns None for any image too short or any offset that would read past the end.
pub fn parse_image_footer(image: &[u8]) -> Option<(Vec<u8>, Vec<u8>)> {
    if image.len() < FOOTER_SIZE {
        return None;
    }
    let f = image.len() - FOOTER_SIZE;
    let u32_at = |o: usize| {
        u32::from_le_bytes([image[f + o], image[f + o + 1], image[f + o + 2], image[f + o + 3]])
            as usize
    };
    let kernel_size = u32_at(28);
    let proof_offset = u32_at(40);
    let proof_size = u32_at(44);
    if kernel_size > image.len() || proof_offset.checked_add(proof_size)? > image.len() {
        return None;
    }
    Some((image[0..kernel_size].to_vec(), image[proof_offset..proof_offset + proof_size].to_vec()))
}
