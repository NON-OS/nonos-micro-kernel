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

use std::fs;

use anyhow::{Context, Result};

use crate::args::Args;
use crate::constants::{FOOTER_SIZE, SIG_ALG_ED25519_MLDSA65};
use crate::footer::create_image_footer;

pub fn write_signed_kernel(
    args: &Args,
    kernel_data: &[u8],
    signature_blob: &[u8],
) -> Result<(u32, u32, usize)> {
    let kernel_size = kernel_data.len() as u32;
    let signature_size = signature_blob.len() as u32;
    let total_size = (kernel_data.len() + signature_blob.len() + FOOTER_SIZE) as u64;
    let footer = create_image_footer(
        kernel_size,
        signature_size,
        SIG_ALG_ED25519_MLDSA65,
        total_size,
        args.rollback_index,
    );
    let mut output_data = kernel_data.to_vec();
    output_data.extend_from_slice(signature_blob);
    output_data.extend_from_slice(&footer);
    fs::write(&args.output, &output_data)
        .with_context(|| format!("Failed to write output: {}", args.output.display()))?;
    Ok((kernel_size, signature_size, output_data.len()))
}
