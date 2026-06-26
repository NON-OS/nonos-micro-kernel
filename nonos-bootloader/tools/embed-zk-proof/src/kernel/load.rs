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
use std::path::Path;

use anyhow::{bail, Context, Result};

pub const ED25519_SIG_SIZE: usize = 64;
pub const FOOTER_SIZE: usize = 64;
pub const FOOTER_MAGIC: [u8; 8] = *b"NONOSIMG";

pub struct SignedKernel {
    pub raw_bytes: Vec<u8>,
    pub kernel_bytes: Vec<u8>,
    pub signature: [u8; ED25519_SIG_SIZE],
    pub rollback_index: u32,
}

pub fn load_signed_kernel(path: &Path) -> Result<SignedKernel> {
    let raw_bytes = fs::read(path)
        .with_context(|| format!("Failed to read signed kernel: {}", path.display()))?;

    if raw_bytes.len() < FOOTER_SIZE + ED25519_SIG_SIZE + 64 {
        bail!("Signed kernel too small");
    }

    let footer_start = raw_bytes.len() - FOOTER_SIZE;
    if &raw_bytes[footer_start..footer_start + 8] != &FOOTER_MAGIC {
        bail!("Missing NONOSIMG footer in signed kernel");
    }

    let kernel_size = u32::from_le_bytes([
        raw_bytes[footer_start + 28],
        raw_bytes[footer_start + 29],
        raw_bytes[footer_start + 30],
        raw_bytes[footer_start + 31],
    ]) as usize;

    if kernel_size > footer_start - ED25519_SIG_SIZE {
        bail!("Invalid kernel size in footer");
    }

    let kernel_bytes = raw_bytes[..kernel_size].to_vec();
    let sig_start = kernel_size;
    let mut signature = [0u8; ED25519_SIG_SIZE];
    signature.copy_from_slice(&raw_bytes[sig_start..sig_start + ED25519_SIG_SIZE]);

    let rollback_index = u32::from_le_bytes([
        raw_bytes[footer_start + 56],
        raw_bytes[footer_start + 57],
        raw_bytes[footer_start + 58],
        raw_bytes[footer_start + 59],
    ]);

    Ok(SignedKernel { raw_bytes, kernel_bytes, signature, rollback_index })
}
