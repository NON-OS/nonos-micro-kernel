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

extern crate alloc;
use alloc::string::{String, ToString};

pub struct Track {
    pub title: String,
    pub artist: String,
    pub format: String,
    pub path: String,
    pub dur_ms: u32,
}

impl Track {
    pub fn from_path(path: &str) -> Self {
        let base = path.rsplit('/').next().unwrap_or(path);
        let (stem, ext) = split_ext(base);
        Track {
            title: prettify(stem),
            artist: String::from("NONOS"),
            format: fmt_label(ext),
            path: path.to_string(),
            dur_ms: 0,
        }
    }
}

fn split_ext(name: &str) -> (&str, &str) {
    match name.rfind('.') {
        Some(i) if i > 0 => (&name[..i], &name[i + 1..]),
        _ => (name, ""),
    }
}

fn fmt_label(ext: &str) -> String {
    let mut s = String::new();
    for b in ext.bytes() {
        s.push(b.to_ascii_uppercase() as char);
    }
    if s.is_empty() {
        s.push_str("RAW");
    }
    s
}

fn prettify(stem: &str) -> String {
    let mut out = String::new();
    let mut cap = true;
    for b in stem.bytes() {
        if b == b'_' || b == b'-' || b == b' ' {
            out.push(' ');
            cap = true;
        } else if cap {
            out.push(b.to_ascii_uppercase() as char);
            cap = false;
        } else {
            out.push(b as char);
        }
    }
    if out.is_empty() {
        out.push_str("Untitled");
    }
    out
}
