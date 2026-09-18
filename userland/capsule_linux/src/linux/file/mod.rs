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


//! The filesystem a guest sees.
//!
//! Every path a guest names is resolved here and reached through the store
//! under this capsule's own identity. The kernel is not involved and holds
//! no filesystem view for a hosted process to inherit.

mod close;
mod dir;
mod dirent;
mod dirents;
mod file;
pub mod flags;
mod open;
mod path;
mod pread;
mod query;
mod read;
mod resolve;
mod seek;
mod slot;
mod stat;
mod statbuf;
mod write;

pub use close::close;
pub use dirents::getdents64;
pub use open::openat;
pub use pread::pread64;
pub use query::{access, getcwd, readlink};
pub use read::read;
pub use seek::lseek;
pub use stat::{fstat, newfstatat};
pub use write::write;
