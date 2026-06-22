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

pub fn commitments(path: &Path) -> Result<Vec<[u8; 32]>> {
    let raw = fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
    if raw.is_empty() || raw.len() % 32 != 0 {
        bail!("commitments file must contain 32-byte commitments");
    }
    let mut out = Vec::with_capacity(raw.len() / 32);
    for chunk in raw.chunks_exact(32) {
        let mut commitment = [0u8; 32];
        commitment.copy_from_slice(chunk);
        out.push(commitment);
    }
    Ok(out)
}
