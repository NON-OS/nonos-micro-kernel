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
//! Where the repository lives, and who is asking.

extern crate alloc;

use alloc::string::String;

use crate::git::VfsStorage;
use crate::term::state::State;

use crate::command::builtin::fs::pid;

/// The git directory inside the current work tree.
pub(super) const GIT_DIR: &str = ".git";

/// Storage rooted at the shell's working directory, which is the work tree.
pub(super) fn storage(state: &mut State) -> VfsStorage {
    let owner = pid(state);
    let cwd = String::from_utf8_lossy(state.cwd.as_bytes()).into_owned();
    VfsStorage::new(owner, &cwd)
}
