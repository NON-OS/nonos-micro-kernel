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

use sha2::{Digest, Sha256};
use std::path::Path;

pub fn hash_file(path: &Path) -> Result<[u8; 32], Box<dyn std::error::Error>> {
    let data = std::fs::read(path)?;
    let out = Sha256::digest(&data);
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&out);
    Ok(hash)
}
