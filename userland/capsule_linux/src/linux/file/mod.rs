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

mod at;
pub(super) mod close;
mod cstr;
mod dir;
mod dirent;
mod dirents;
mod dirops;
mod epoll;
mod epoll_wait;
pub mod flags;
mod fsync;
mod memfd;
mod memfd_map;
mod meta;
mod open;
mod path;
mod pread;
mod read;
mod regular;
mod rename;
mod resolve;
mod root;
mod seek;
mod slot;
mod store;
mod store_name;
mod timerfd;
mod timerfd_read;
mod write;

pub use close::close;
pub use cstr::read_cstr;
pub use dirents::getdents64;
pub use dirops::{mkdirat, rmdir, unlinkat};
pub use epoll::{epoll_create, epoll_ctl};
pub use epoll_wait::epoll_wait;
pub use fsync::fsync;
pub use memfd::{ftruncate, is_memfd, memfd_create};
pub use memfd_map::{mapped_at, set_mapped, staged};
pub use meta::{
    access, chmod, faccessat, fchmod, fchmodat, fstat, look, newfstatat, readlink, statfs, statx,
};
pub use open::openat;
pub use path::read_path;
pub use pread::pread64;
pub use read::read;
pub use rename::rename;
pub use resolve::{key, visible};
pub use seek::lseek;
pub use slot::{install, MAX_FDS};
pub use store::{read as store_read, write as store_write};
pub use timerfd::{timerfd_create, timerfd_settime};
pub use timerfd_read::read as timerfd_read;
pub use write::write;
