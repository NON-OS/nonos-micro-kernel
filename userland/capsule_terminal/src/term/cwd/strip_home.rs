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

/// The part of `cwd` that sits under `home`, so a caller can print it behind
/// a `~`. `None` when there is no home to speak of or the path is not inside
/// it -- a bare prefix match is not enough, because `/homework` starts with
/// `/home` and is nowhere near it.
pub fn strip_home<'a>(cwd: &'a [u8], home: &[u8]) -> Option<&'a [u8]> {
    if home.is_empty() || home == b"/" || !cwd.starts_with(home) {
        return None;
    }
    let tail = &cwd[home.len()..];
    (tail.is_empty() || tail[0] == b'/').then_some(tail)
}
