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
//! The error a repository operation reports.

use crate::commit::CommitError;
use crate::index::IndexError;
use crate::odb::OdbError;
use crate::pack::PackError;
use crate::storage::StorageError;
use crate::tree::TreeError;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RepoError {
    /// A repository already exists where one was to be created.
    Exists,
    /// The path is not a repository.
    NotARepository,
    /// The object database refused the read or write.
    Odb(OdbError),
    /// The filesystem refused the read or write.
    Storage(StorageError),
    /// An object that should have been a tree was not a valid one.
    Tree(TreeError),
    /// An object that should have been a commit was not a valid one.
    Commit(CommitError),
    /// An object turned out to be a different kind than the reference to it
    /// implied, so the repository is inconsistent.
    WrongKind,
    /// `HEAD` was missing or unreadable.
    NoHead,
    /// The index file was damaged.
    Index(IndexError),
    /// A fetched pack could not be read.
    Pack(PackError),
}
