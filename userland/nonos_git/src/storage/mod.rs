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

//! The filesystem a repository lives on.
//!
//! Everything above this is pure: the repository decides which paths to read
//! and write and what bytes go in them, and a `Storage` carries that out. The
//! terminal passes one backed by the VFS capsule; the tests pass one backed by
//! a real directory, so the code that runs in the OS is the code shown to
//! produce repositories git can read.

mod error;
mod traits;

pub use error::StorageError;
pub use traits::Storage;
