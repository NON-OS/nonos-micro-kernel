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
//! Lifting the errors of the layers below into `RepoError`.

use crate::commit::CommitError;
use crate::odb::OdbError;
use crate::pack::PackError;
use crate::storage::StorageError;
use crate::tree::TreeError;

use super::kind::RepoError;

impl From<OdbError> for RepoError {
    fn from(e: OdbError) -> RepoError {
        RepoError::Odb(e)
    }
}

impl From<StorageError> for RepoError {
    fn from(e: StorageError) -> RepoError {
        RepoError::Storage(e)
    }
}

impl From<TreeError> for RepoError {
    fn from(e: TreeError) -> RepoError {
        RepoError::Tree(e)
    }
}

impl From<CommitError> for RepoError {
    fn from(e: CommitError) -> RepoError {
        RepoError::Commit(e)
    }
}

impl From<PackError> for RepoError {
    fn from(e: PackError) -> RepoError {
        RepoError::Pack(e)
    }
}
