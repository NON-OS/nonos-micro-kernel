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
//! Joining a repository path onto the work tree.

extern crate alloc;

use alloc::string::String;

/// Both sides may carry a slash, so this trims rather than concatenating
/// blindly: a doubled separator names a different path to the VFS.
pub(super) fn join(root: &str, path: &str) -> String {
    let base = root.trim_end_matches('/');
    let rel = path.trim_start_matches('/');
    let mut out = String::with_capacity(base.len() + rel.len() + 1);
    out.push_str(base);
    out.push('/');
    out.push_str(rel);
    out
}
