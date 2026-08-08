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

//! Repository operations: creating one, staging into it, recording a commit
//! and reading its history.

mod add;
mod checkout;
mod clone;
mod commit_tree;
pub(crate) mod error;
mod init;
mod log;
mod push;
mod read_index;
mod request;
mod tree_build;
mod write_index;

pub use add::add;
pub use checkout::checkout;
pub use clone::{clone_into, store_pack, CloneRequest};
pub use commit_tree::commit;
pub use error::RepoError;
pub use init::init;
pub use log::{log, LogEntry};
pub use push::objects_to_send;
pub use read_index::read_index;
pub use request::CommitRequest;
pub use tree_build::write_tree;
pub use write_index::write_index;
