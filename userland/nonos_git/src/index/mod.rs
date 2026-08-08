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

//! The staging index: what the next commit will contain.
//!
//! Git stores this as the binary `DIRC` file version 2. The format is fixed and
//! checksummed, and `git` reads the same file we write, so `add` here is `add`
//! git agrees with.

mod encode;
pub(crate) mod entry;
pub(crate) mod error;
mod mode_word;
mod read;
mod stage;

pub use encode::encode;
pub use entry::IndexEntry;
pub use error::IndexError;
pub use read::parse;
pub use stage::stage;
