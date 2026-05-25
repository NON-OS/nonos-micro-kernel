// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

extern crate alloc;

use alloc::string::String;

pub(in crate::elf::loader::core) fn read_string_from_data_limited(
    data: &[u8],
    offset: usize,
    max_len: usize,
) -> String {
    let mut result = String::new();
    let end = offset.saturating_add(max_len).min(data.len());
    let mut pos = offset;
    while pos < end && data[pos] != 0 && result.len() < 256 {
        result.push(data[pos] as char);
        pos += 1;
    }
    result
}
