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

use std::{fs, path::Path};

use super::entry::Entry;
use super::parse_caps::parse_caps;

pub fn read_policy(path: &Path) -> Result<Vec<Entry>, String> {
    let data = fs::read_to_string(path).map_err(|e| format!("policy file: {e}"))?;
    let mut out = Vec::new();
    for line in data.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        if parts.is_empty() {
            continue;
        }
        if parts.len() != 3 {
            return Err("policy entry must be: name path caps".into());
        }
        out.push(Entry { capsule: parts[1].into(), caps: parse_caps(parts[2])? });
    }
    Ok(out)
}
