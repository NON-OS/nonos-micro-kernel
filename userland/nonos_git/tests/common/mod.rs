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

//! Shared test fixtures.
//!
//! Each integration test binary compiles this module but uses only the parts
//! it needs, so unused items here are expected rather than dead.

#![allow(dead_code, unused_imports)]

mod build;
mod git_cmd;
mod local_git;
mod receive;
mod replay;
mod scratch;
mod storage;

pub use build::{build_repo, signature};
pub use git_cmd::{git, git_available};
pub use local_git::LocalGit;
pub use receive::receive_pack;
pub use replay::Replay;
pub use scratch::Scratch;
pub use storage::DirStorage;
