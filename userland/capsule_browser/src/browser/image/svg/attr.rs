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

// Value of `name="v"` (or single-quoted) inside a tag's attribute text.
pub(super) fn attr<'a>(attrs: &'a str, name: &str) -> Option<&'a str> {
    let bytes = attrs.as_bytes();
    let mut i = 0;
    while i < attrs.len() {
        let rest = &attrs[i..];
        let hit = rest.find(name)?;
        let at = i + hit;
        // Must sit on a word boundary and be followed by =.
        let before_ok = at == 0 || !bytes[at - 1].is_ascii_alphanumeric() && bytes[at - 1] != b'-';
        let after = attrs[at + name.len()..].trim_start();
        if before_ok {
            if let Some(v) = after.strip_prefix('=') {
                let v = v.trim_start();
                let quote = v.as_bytes().first().copied();
                if quote == Some(b'"') || quote == Some(b'\'') {
                    let q = quote.unwrap_or(b'"') as char;
                    let inner = &v[1..];
                    return inner.find(q).map(|e| &inner[..e]);
                }
                let end = v.find(char::is_whitespace).unwrap_or(v.len());
                return Some(&v[..end]);
            }
        }
        i = at + name.len();
    }
    None
}

// A property from an inline style attribute: "fill:#fff;stroke:none".
pub(super) fn style_prop<'a>(style: &'a str, name: &str) -> Option<&'a str> {
    for decl in style.split(';') {
        let Some((k, v)) = decl.split_once(':') else { continue };
        if k.trim().eq_ignore_ascii_case(name) {
            return Some(v.trim());
        }
    }
    None
}
