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

use alloc::string::ToString;
use alloc::vec::Vec;

use crate::browser::css::decl::Decl;

pub fn parse_decls(block: &str) -> Vec<Decl> {
    let mut out: Vec<Decl> = Vec::new();
    for d in block.split(';') {
        let Some(colon) = d.find(':') else {
            continue;
        };
        let name = d[..colon].trim().to_ascii_lowercase();
        let value = d[colon + 1..].trim().trim_end_matches("!important").trim().to_string();
        if !name.is_empty() && !value.is_empty() {
            out.push(Decl { name, value });
        }
        if out.len() >= 256 {
            break;
        }
    }
    out
}
