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

use std::{collections::BTreeMap, path::PathBuf};

pub fn map_by_name(paths: &[PathBuf], suffix: &str) -> Result<BTreeMap<String, PathBuf>, String> {
    let mut out = BTreeMap::new();
    for path in paths {
        let name = path.file_name().and_then(|n| n.to_str()).ok_or("bad file name")?;
        let key = name.strip_suffix(suffix).ok_or("bad artifact suffix")?;
        if out.insert(key.to_string(), path.clone()).is_some() {
            return Err(format!("duplicate capsule artifact: {key}"));
        }
    }
    Ok(out)
}
