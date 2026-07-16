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

// The by-path filesystem operations std exposes as free functions, one to a
// file: stat and its relatives, unlink/rmdir/rename, the recursive
// remove_dir_all the store has no native form of, canonicalize, copy, the
// unsupported link operations, and the no-op metadata setters.

mod canonicalize;
mod copy;
mod links;
mod meta;
mod remove_dir_all;
mod rename;
mod rmdir;
mod stat;
mod unlink;

pub use canonicalize::canonicalize;
pub use copy::copy;
pub use links::{link, readlink, symlink};
pub use meta::{set_perm, set_times, set_times_nofollow};
pub use remove_dir_all::remove_dir_all;
pub use rename::rename;
pub use rmdir::rmdir;
pub use stat::{exists, lstat, stat};
pub use unlink::unlink;
