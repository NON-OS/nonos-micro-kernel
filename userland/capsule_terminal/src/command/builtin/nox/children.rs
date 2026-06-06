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

use alloc::string::String;
use alloc::vec::Vec;

pub fn children(prefix: &str, paths: &[String]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for full in paths {
        let rel = match full.strip_prefix(prefix) {
            Some(r) if !r.is_empty() => r,
            _ => continue,
        };
        let mut name = match rel.split('/').next() {
            Some(s) if !s.is_empty() => String::from(s),
            _ => continue,
        };
        if rel.len() > name.len() {
            name.push('/');
        }
        if !out.iter().any(|e| e == &name) {
            out.push(name);
        }
    }
    out
}
