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

pub fn path_without_query(path: &str) -> &str {
    let q = path.find('?');
    let h = path.find('#');
    match (q, h) {
        (Some(a), Some(b)) => &path[..core::cmp::min(a, b)],
        (Some(a), None) => &path[..a],
        (None, Some(b)) => &path[..b],
        (None, None) => path,
    }
}
