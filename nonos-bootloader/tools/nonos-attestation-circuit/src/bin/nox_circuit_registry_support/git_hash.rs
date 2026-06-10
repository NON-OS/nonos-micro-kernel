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

use std::path::Path;
use std::process::Command;

pub fn git_hash32(dir: &Path, spec: &str) -> Result<[u8; 32], String> {
    let output = Command::new("git")
        .current_dir(dir)
        .args(["rev-parse", spec])
        .output()
        .map_err(|e| format!("failed to run git: {e}"))?;
    if !output.status.success() {
        return Err(format!("git rev-parse {spec} failed"));
    }
    let text = String::from_utf8_lossy(&output.stdout);
    let text = text.trim();
    if text.len() < 40 {
        return Err(format!("invalid git hash for {spec}"));
    }
    let mut hash = [0u8; 32];
    for (i, chunk) in text.as_bytes().chunks(2).take(32).enumerate() {
        if chunk.len() == 2 {
            hash[i] =
                u8::from_str_radix(std::str::from_utf8(chunk).unwrap_or("00"), 16).unwrap_or(0);
        }
    }
    Ok(hash)
}
