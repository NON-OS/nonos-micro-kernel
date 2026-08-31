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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MediaKind {
    Avi,
    Mp4,
    Mkv,
    Mov,
}

const TABLE: [(&str, MediaKind); 4] = [
    (".avi", MediaKind::Avi),
    (".mp4", MediaKind::Mp4),
    (".mkv", MediaKind::Mkv),
    (".mov", MediaKind::Mov),
];

impl MediaKind {
    pub fn label(self) -> &'static str {
        match self {
            MediaKind::Avi => "AVI",
            MediaKind::Mp4 => "MP4",
            MediaKind::Mkv => "MKV",
            MediaKind::Mov => "MOV",
        }
    }

    pub fn decodable(self) -> bool {
        matches!(self, MediaKind::Avi)
    }
}

pub fn kind_of(name: &str) -> Option<MediaKind> {
    let bytes = name.as_bytes();
    for (ext, kind) in TABLE {
        if bytes.len() <= ext.len() {
            continue;
        }
        let (stem, tail) = bytes.split_at(bytes.len() - ext.len());
        if !stem.is_empty() && tail.eq_ignore_ascii_case(ext.as_bytes()) {
            return Some(kind);
        }
    }
    None
}

pub fn strip_ext(name: &str) -> &str {
    match kind_of(name) {
        Some(_) => &name[..name.len() - 4],
        None => name,
    }
}
