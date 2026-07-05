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

use super::entries::Entry;

// Broad content category, used to colour a row and to group the type sort.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    Dir,
    Code,
    Image,
    Doc,
    Archive,
    Exec,
    Other,
}

// The extension of a name, empty when there is none. A leading dot marks a
// hidden file rather than an extension, so `.gitignore` has no extension.
pub fn ext(label: &str) -> &str {
    let name = label.trim_end_matches('/');
    match name.rfind('.') {
        Some(i) if i > 0 && i + 1 < name.len() => &name[i + 1..],
        _ => "",
    }
}

pub fn kind_of(entry: &Entry) -> Kind {
    if entry.is_dir {
        return Kind::Dir;
    }
    match ext(&entry.label) {
        "rs" | "c" | "h" | "cpp" | "py" | "js" | "ts" | "go" | "sh" | "toml" | "json" | "md"
        | "html" | "css" | "lua" => Kind::Code,
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "svg" | "webp" | "ico" => Kind::Image,
        "txt" | "pdf" | "log" | "cfg" | "conf" | "ini" | "csv" => Kind::Doc,
        "zip" | "tar" | "gz" | "xz" | "bz2" | "7z" | "zst" => Kind::Archive,
        "elf" | "bin" | "exe" | "wasm" | "so" | "a" => Kind::Exec,
        _ => Kind::Other,
    }
}

// Row colour for a category, so the listing reads by content at a glance.
pub fn color(kind: Kind) -> u32 {
    match kind {
        Kind::Dir => 0xFF6CE08C,
        Kind::Code => 0xFF7FB4FF,
        Kind::Image => 0xFFD08CF0,
        Kind::Doc => 0xFFD7E2F2,
        Kind::Archive => 0xFFE0B060,
        Kind::Exec => 0xFFE0785C,
        Kind::Other => 0xFFA8B6CC,
    }
}
