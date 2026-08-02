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

//! What a tree entry name may be.

extern crate alloc;

use alloc::string::String;

use super::parse::TreeError;

/// A single path component. Refusing `/`, `.` and `..` is what stops a hostile
/// tree writing outside the work tree when it is checked out.
pub(super) fn check_name(bytes: &[u8]) -> Result<String, TreeError> {
    if bytes.is_empty() || bytes == b"." || bytes == b".." {
        return Err(TreeError::Name);
    }
    if bytes.contains(&b'/') || bytes.contains(&0) {
        return Err(TreeError::Name);
    }
    core::str::from_utf8(bytes).map(String::from).map_err(|_| TreeError::Name)
}
