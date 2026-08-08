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

//! Why an object could not be stored or loaded.

use crate::storage::StorageError;
use crate::zlib::InflateError;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OdbError {
    /// No object with that id is in the database.
    NotFound,
    /// The filesystem refused the read or write.
    Storage(StorageError),
    /// The stored bytes did not inflate.
    Corrupt(InflateError),
    /// The inflated bytes were not a valid `<type> <size>\0<content>` object.
    Malformed,
    /// The object's content does not hash to the id it was stored under, so
    /// the database has been damaged or tampered with.
    IdMismatch,
}

impl From<StorageError> for OdbError {
    fn from(e: StorageError) -> OdbError {
        match e {
            StorageError::NotFound => OdbError::NotFound,
            other => OdbError::Storage(other),
        }
    }
}
