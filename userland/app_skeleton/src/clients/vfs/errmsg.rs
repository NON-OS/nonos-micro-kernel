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

// Turn a vfs errno reply into a short human message so a caller can surface the
// real reason a request failed instead of one generic string. Values mirror the
// service's errno table.
pub fn errmsg(status: i32) -> &'static str {
    match status {
        -2 => "not found",
        -9 => "bad handle",
        -13 => "access denied",
        -17 => "already exists",
        -21 => "is a directory",
        -22 => "invalid request",
        -28 => "no space left",
        -39 => "directory not empty",
        -90 => "too large",
        _ => "operation failed",
    }
}
