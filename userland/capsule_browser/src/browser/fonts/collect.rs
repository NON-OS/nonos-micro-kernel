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

use super::key::family_key;
use super::pick_src::pick_src;

// Every @font-face in the sheet as (family key, source url). Only formats the
// rasterizer can load are picked; a face whose sources are all unsupported is
// skipped and its text falls back to the built-in face.
pub fn collect_font_faces(css: &str) -> Vec<(u32, String)> {
    let mut out: Vec<(u32, String)> = Vec::new();
    let mut rest = css;
    while let Some(pos) = rest.find("@font-face") {
        rest = &rest[pos + "@font-face".len()..];
        let Some(open) = rest.find('{') else { break };
        let Some(close) = rest[open..].find('}') else { break };
        let body = &rest[open + 1..open + close];
        rest = &rest[open + close + 1..];
        let Some(family) = decl_value(body, "font-family") else { continue };
        let Some(url) = decl_value(body, "src").and_then(|v| pick_src(v)) else { continue };
        let key = family_key(family);
        if key == 0 {
            continue;
        }
        // One face per family; the regular weight wins over whichever weight
        // the sheet happened to declare first.
        let regular = decl_value(body, "font-weight").map(|w| w.trim() == "400").unwrap_or(true);
        match out.iter_mut().find(|(k, _)| *k == key) {
            Some(slot) if regular => slot.1 = url,
            Some(_) => {}
            None => out.push((key, url)),
        }
    }
    out
}

// The value of the first `name:` declaration in a rule body. Splitting on the
// first colon keeps urls inside the value whole.
fn decl_value<'a>(body: &'a str, name: &str) -> Option<&'a str> {
    for decl in body.split(';') {
        let Some((n, v)) = decl.split_once(':') else { continue };
        if n.trim().eq_ignore_ascii_case(name) {
            return Some(v.trim());
        }
    }
    None
}
