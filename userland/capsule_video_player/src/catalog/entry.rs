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

const EXT: &[u8; 4] = b".avi";

pub fn file_name(path: &str) -> &str {
    match path.rfind('/') {
        Some(i) => &path[i + 1..],
        None => path,
    }
}

pub fn is_playable(path: &str) -> bool {
    let name = file_name(path).as_bytes();
    if name.len() <= EXT.len() {
        return false;
    }
    let (stem, ext) = name.split_at(name.len() - EXT.len());
    !stem.is_empty() && ext.eq_ignore_ascii_case(EXT)
}
