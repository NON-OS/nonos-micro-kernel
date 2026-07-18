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

use super::error::ImageValidationError;
use super::signature::validate_signature_size;
use crate::image_format::parse::{parse_image_footer, ParsedImage};
use crate::image_format::types::{HashAlgorithm, SignatureAlgorithm};

pub const MIN_KERNEL_SIZE: usize = 64;
pub const ELF_MAGIC: [u8; 4] = [0x7f, b'E', b'L', b'F'];
pub const ZK_PROOF_MAGIC: [u8; 4] = [0x4E, 0xC3, 0x5A, 0x50];
// The transparent-STARK kernel self-attestation trailer carries this magic
// instead of the boot-binding block above.
pub const STARK_TRAILER_MAGIC: [u8; 8] = *b"NZKSTRK1";
pub const MIN_ZK_PROOF_SIZE: usize = 272;

pub fn validate_image(data: &[u8]) -> Result<ParsedImage<'_>, ImageValidationError> {
    let parsed = parse_image_footer(data)?;

    validate_kernel_payload(&parsed)?;
    validate_signature_size(parsed.signature_bytes, parsed.signature_algorithm)?;
    validate_proof_if_present(&parsed)?;
    validate_algorithm_consistency(&parsed)?;

    Ok(parsed)
}

fn validate_kernel_payload(parsed: &ParsedImage<'_>) -> Result<(), ImageValidationError> {
    if parsed.kernel_bytes.len() < MIN_KERNEL_SIZE {
        return Err(ImageValidationError::KernelTooSmall);
    }

    if parsed.kernel_bytes.len() < 4 {
        return Err(ImageValidationError::KernelNotElf);
    }

    if &parsed.kernel_bytes[0..4] != &ELF_MAGIC {
        return Err(ImageValidationError::KernelNotElf);
    }

    Ok(())
}

fn validate_proof_if_present(parsed: &ParsedImage<'_>) -> Result<(), ImageValidationError> {
    if let Some(proof_bytes) = parsed.proof_bytes {
        if proof_bytes.len() < MIN_ZK_PROOF_SIZE {
            return Err(ImageValidationError::ProofTooSmall);
        }

        // Accept either proof-block format: the earlier boot-binding block or the
        // transparent-STARK self-attestation trailer. This is a shape check; the
        // proof itself is verified in kernel_verify, which knows which one to run.
        let is_curve = &proof_bytes[0..4] == &ZK_PROOF_MAGIC;
        let is_stark = proof_bytes.len() >= 8 && &proof_bytes[0..8] == &STARK_TRAILER_MAGIC;
        if !is_curve && !is_stark {
            return Err(ImageValidationError::ProofMagicInvalid);
        }
    }

    Ok(())
}

fn validate_algorithm_consistency(parsed: &ParsedImage<'_>) -> Result<(), ImageValidationError> {
    if parsed.hash_algorithm != HashAlgorithm::Blake3_256 {
        return Err(ImageValidationError::HashAlgorithmMismatch);
    }
    if parsed.signature_algorithm != SignatureAlgorithm::Ed25519MlDsa65 {
        return Err(ImageValidationError::SignatureAlgorithmMismatch);
    }

    Ok(())
}
