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
//! The text of a remote section.

extern crate alloc;

use alloc::format;
use alloc::string::String;

/// A `[remote "name"]` section, in the layout git writes.
///
/// The fetch refspec is what tells git which remote branches map to which
/// local tracking refs. It is written so the repository reads the same to git
/// as one git cloned itself, even though nothing here consumes it yet.
pub(super) fn remote_section(name: &str, url: &str) -> String {
    format!("[remote \"{name}\"]\n\turl = {url}\n\tfetch = +refs/heads/*:refs/remotes/{name}/*\n")
}
