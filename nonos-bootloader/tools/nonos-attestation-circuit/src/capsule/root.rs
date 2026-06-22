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

pub fn read_root(path: &Path) -> Result<[u8; 32], String> {
    let bytes = fs::read(path).map_err(|e| e.to_string())?;
    if bytes.len() != 32 {
        return Err("root must be exactly 32 bytes".into());
    }
    bytes.as_slice().try_into().map_err(|_| "root".into())
}
