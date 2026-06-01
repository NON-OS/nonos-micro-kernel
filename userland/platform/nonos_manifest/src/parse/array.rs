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

use super::strings::unquote;

pub fn parse_str_array(value: &str) -> Result<Vec<String>, String> {
    let v = value.trim();
    if !(v.starts_with('[') && v.ends_with(']')) {
        return Err(format!("expected array, got: {value}"));
    }
    let inner = v[1..v.len() - 1].trim();
    if inner.is_empty() {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();
    for part in inner.split(',') {
        let p = part.trim();
        if !p.is_empty() {
            out.push(unquote(p)?);
        }
    }
    Ok(out)
}
