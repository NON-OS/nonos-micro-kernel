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

pub fn read_meta(pkg: &Path) -> Result<(String, (u32, u32, u32), String, String), String> {
    let text = fs::read_to_string(pkg.join("package.meta")).map_err(|e| format!("read meta: {e}"))?;
    let mut lines = text.lines();
    let name = lines.next().ok_or("meta: missing name")?.to_string();
    let mut v = lines.next().ok_or("meta: missing version")?.split('.');
    let a = v.next().and_then(|x| x.parse().ok()).ok_or("meta: bad version")?;
    let b = v.next().and_then(|x| x.parse().ok()).ok_or("meta: bad version")?;
    let c = v.next().and_then(|x| x.parse().ok()).ok_or("meta: bad version")?;
    let target = lines.next().ok_or("meta: missing target")?.to_string();
    let namespace = lines.next().ok_or("meta: missing namespace")?.to_string();
    Ok((name, (a, b, c), target, namespace))
}
