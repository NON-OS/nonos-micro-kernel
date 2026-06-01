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

use std::fs;
use std::path::Path;

use super::encode::encode;
use super::entry::RegistryEntry;

pub fn save(index: &Path, entries: &[RegistryEntry]) -> Result<(), String> {
    let mut sorted: Vec<&RegistryEntry> = entries.iter().collect();
    sorted.sort_by(|a, b| a.name.cmp(&b.name).then(a.version.cmp(&b.version)));
    let mut body = String::new();
    for entry in sorted {
        body.push_str(&encode(entry));
        body.push('\n');
    }
    fs::write(index, body).map_err(|e| format!("write index: {e}"))
}
