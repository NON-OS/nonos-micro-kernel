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

use super::entry::RegistryEntry;

pub fn decode(line: &str) -> Option<RegistryEntry> {
    let f: Vec<&str> = line.split('\t').collect();
    if f.len() != 5 {
        return None;
    }
    let mut v = f[1].split('.');
    let a = v.next()?.parse().ok()?;
    let b = v.next()?.parse().ok()?;
    let c = v.next()?.parse().ok()?;
    if v.next().is_some() {
        return None;
    }
    Some(RegistryEntry {
        name: f[0].to_string(),
        version: (a, b, c),
        target: f[2].to_string(),
        namespace: f[3].to_string(),
        cert_fingerprint: f[4].to_string(),
    })
}
