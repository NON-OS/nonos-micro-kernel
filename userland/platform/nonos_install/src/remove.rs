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

use nonos_package::package_dir;

pub fn remove(store: &Path, name: &str, version: (u32, u32, u32)) -> Result<(), String> {
    let dir = package_dir(store, name, version);
    if !dir.exists() {
        return Err(format!("not installed: {name}"));
    }
    fs::remove_dir_all(&dir).map_err(|e| format!("remove failed: {e}"))
}
