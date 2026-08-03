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
//! One row of `git verify-pack -v`.

use nonos_git::ObjectId;

/// The columns are id, type, size, size-in-pack, offset. Anything else in the
/// output, such as the chain histogram at the end, has no id in the first
/// column and is skipped.
pub fn git_row(line: &str) -> Option<(ObjectId, u64)> {
    let mut parts = line.split_whitespace();
    let id = parts.next()?;
    if id.len() != 40 {
        return None;
    }
    let offset = parts.nth(3)?.parse::<u64>().ok()?;
    Some((ObjectId::from_hex(id)?, offset))
}
