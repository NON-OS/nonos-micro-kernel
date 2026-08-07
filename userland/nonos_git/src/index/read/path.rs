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

//! What an index path may be.

use crate::index::error::IndexError;

/// Relative, and staying inside the work tree. Refusing an absolute path or a
/// `..` component stops a hostile index writing outside it on checkout.
pub(super) fn check_path(path: &str) -> Result<(), IndexError> {
    if path.is_empty() || path.starts_with('/') {
        return Err(IndexError::Entry);
    }
    if path.split('/').any(|p| p.is_empty() || p == "." || p == "..") {
        return Err(IndexError::Entry);
    }
    Ok(())
}
