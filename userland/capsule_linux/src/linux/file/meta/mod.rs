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

//! What the store knows about a name, and what a program may do with it.

mod perms;
mod query;
pub(super) mod stat;
mod statbuf;
mod statfs;
mod statx;

pub use perms::{chmod, faccessat, fchmod, fchmodat};
pub use query::{access, readlink};
pub use stat::{fstat, look, newfstatat};
pub use statfs::statfs;
pub use statx::statx;
