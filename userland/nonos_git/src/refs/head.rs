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

//! What `HEAD` points at.

extern crate alloc;

use alloc::string::String;

use crate::oid::ObjectId;

/// A repository is either on a branch or detached at a commit.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Head {
    /// On a branch, named as it appears after `refs/heads/`. The branch file
    /// may not exist yet, which is the state a fresh repository is in before
    /// its first commit.
    Branch(String),
    /// Detached: `HEAD` holds a commit id directly.
    Detached(ObjectId),
}
