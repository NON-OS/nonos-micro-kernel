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
use super::file_ext::ext;
use super::filetype::Kind;

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
