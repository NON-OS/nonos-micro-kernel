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

//! The commit object: a tree, its parents, who made it and why.

mod encode;
pub(crate) mod lines;
pub(crate) mod offset;
mod read;
pub(crate) mod sig;
pub(crate) mod types;

pub use encode::encode;
pub use read::{parse, CommitError};
pub use sig::Signature;
pub use types::Commit;
