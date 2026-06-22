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

pub fn challenge_file(path: &Path, kernel_hash: &[u8; 32]) -> Result<([u8; 32], [u8; 32], u64)> {
    let raw = fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
    if raw.len() != 104 {
        bail!("challenge file must be exactly 104 bytes");
    }
    if &raw[0..32] != kernel_hash.as_slice() {
        bail!("challenge kernel hash mismatch");
    }
    let mut nonce = [0u8; 32];
    let mut machine = [0u8; 32];
    nonce.copy_from_slice(&raw[32..64]);
    machine.copy_from_slice(&raw[64..96]);
    let timestamp = u64::from_be_bytes(raw[96..104].try_into()?);
    Ok((nonce, machine, timestamp))
}
