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

use super::bytes::parse_footer_bytes;
use super::error::ParseError;
use super::overlap::validate_no_overlap;
use crate::image_format::footer::{ImageFooter, FOOTER_MAGIC, FOOTER_SIZE};
use crate::image_format::types::{HashAlgorithm, SignatureAlgorithm};

#[derive(Debug)]
pub struct ParsedImage<'a> {
    pub footer: ImageFooter,
    pub kernel_bytes: &'a [u8],
    pub signature_bytes: &'a [u8],
    pub proof_bytes: Option<&'a [u8]>,
    pub hash_algorithm: HashAlgorithm,
    pub signature_algorithm: SignatureAlgorithm,
}

pub fn parse_image_footer(data: &[u8]) -> Result<ParsedImage<'_>, ParseError> {
    if data.len() < FOOTER_SIZE {
        return Err(ParseError::ImageTooSmall);
    }

    let footer_start = data.len() - FOOTER_SIZE;
    let footer = parse_footer_bytes(&data[footer_start..])?;

    validate_footer_fields(&footer, data.len())?;

    let hash_alg = HashAlgorithm::from_u8(footer.hash_algorithm)
        .ok_or(ParseError::HashAlgorithmUnsupported)?;

    let sig_alg = SignatureAlgorithm::from_u8(footer.signature_algorithm)
        .ok_or(ParseError::SignatureAlgorithmUnsupported)?;

    validate_no_overlap(&footer)?;
    extract_image_regions(data, footer, hash_alg, sig_alg)
}

fn validate_footer_fields(footer: &ImageFooter, file_len: usize) -> Result<(), ParseError> {
    if !footer.is_valid_magic() {
        return Err(ParseError::FooterMagicInvalid);
    }
    if !footer.is_supported_version() {
        return Err(ParseError::FooterVersionUnsupported);
    }
    if footer.total_image_size != file_len as u64 {
        return Err(ParseError::TotalSizeMismatch);
    }
    Ok(())
}

fn extract_image_regions<'a>(
    data: &'a [u8],
    footer: ImageFooter,
    hash_alg: HashAlgorithm,
    sig_alg: SignatureAlgorithm,
) -> Result<ParsedImage<'a>, ParseError> {
    let footer_start = data.len() - FOOTER_SIZE;
    let k_end = footer.kernel_end().ok_or(ParseError::KernelOffsetOverflow)? as usize;
    let s_end = footer.signature_end().ok_or(ParseError::SignatureOffsetOverflow)? as usize;

    if k_end > footer_start || s_end > footer_start {
        return Err(ParseError::KernelOutOfBounds);
    }

    let kernel_bytes = &data[footer.kernel_offset as usize..k_end];
    let sig_bytes = &data[footer.signature_offset as usize..s_end];
    let proof_bytes = extract_proof_bytes(data, &footer, footer_start)?;

    Ok(ParsedImage {
        footer,
        kernel_bytes,
        signature_bytes: sig_bytes,
        proof_bytes,
        hash_algorithm: hash_alg,
        signature_algorithm: sig_alg,
    })
}

fn extract_proof_bytes<'a>(
    data: &'a [u8],
    footer: &ImageFooter,
    footer_start: usize,
) -> Result<Option<&'a [u8]>, ParseError> {
    if !footer.has_zk_proof() || footer.proof_size == 0 {
        return Ok(None);
    }
    let p_end = footer.proof_end().ok_or(ParseError::ProofOffsetOverflow)? as usize;
    if p_end > footer_start {
        return Err(ParseError::ProofOutOfBounds);
    }
    Ok(Some(&data[footer.proof_offset as usize..p_end]))
}

pub fn has_production_footer(data: &[u8]) -> bool {
    if data.len() < FOOTER_SIZE {
        return false;
    }
    let footer_start = data.len() - FOOTER_SIZE;
    &data[footer_start..footer_start + 8] == &FOOTER_MAGIC
}
