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


//! Calls that name a file or a descriptor.

use crate::linux::abi::{nr, nr_path as np};
use crate::linux::call;
use crate::linux::file;
use crate::linux::file::flags;
use crate::linux::guest::Guest;

pub fn file_ops(guest: &mut Guest, tid: u32, nr: u64, a: [u64; 6]) -> Option<u64> {
    let _ = tid;
    Some(match nr {
        nr::WRITE => call::write(guest, a[0], a[1], a[2]),
        nr::WRITEV => call::writev(guest, a[0], a[1], a[2]),
        nr::READ => call::read(guest, a[0], a[1], a[2]),
        nr::CLOSE => call::close(guest, a[0]),
        nr::MEMFD_CREATE => file::memfd_create(guest),
        nr::FTRUNCATE => file::ftruncate(guest, a[0], a[1]),
        nr::OPENAT => file::openat(guest, a[0], a[1], a[2]),
        nr::OPEN => file::openat(guest, flags::AT_FDCWD, a[0], a[1]),
        nr::LSEEK => file::lseek(guest, a[0], a[1], a[2]),
        nr::FSTAT => file::fstat(guest, a[0], a[1]),
        nr::STAT | nr::LSTAT => file::newfstatat(guest, flags::AT_FDCWD, a[0], a[1]),
        nr::NEWFSTATAT => file::newfstatat(guest, a[0], a[1], a[2]),
        nr::GETDENTS64 => file::getdents64(guest, a[0], a[1], a[2]),
        nr::EPOLL_CREATE1 => file::epoll_create(guest),
        nr::PIPE => call::pipe2(guest, a[0], 0),
        nr::PIPE2 => call::pipe2(guest, a[0], a[1]),
        nr::DUP => call::dup(guest, a[0]),
        nr::DUP2 | nr::DUP3 => call::dup2(guest, a[0], a[1]),
        nr::EPOLL_CTL => file::epoll_ctl(guest, a[0], a[1], a[2], a[3]),
        nr::EPOLL_PWAIT => file::epoll_wait(guest, a[0], a[1], a[2]),
        nr::TIMERFD_CREATE => file::timerfd_create(guest),
        nr::TIMERFD_SETTIME => file::timerfd_settime(guest, a[0], a[2]),
        nr::PREAD64 => file::pread64(guest, a[0], a[1], a[2], a[3]),
        nr::GETCWD => call::getcwd(guest, a[0], a[1]),
        np::CHDIR => call::chdir(guest, a[0]),
        np::FCHDIR => call::fchdir(guest, a[0]),
        np::MKDIR => file::mkdirat(guest, flags::AT_FDCWD, a[0]),
        np::MKDIRAT => file::mkdirat(guest, a[0], a[1]),
        np::RMDIR => file::rmdir(guest, a[0]),
        np::UNLINK => file::unlinkat(guest, flags::AT_FDCWD, a[0], 0),
        np::UNLINKAT => file::unlinkat(guest, a[0], a[1], a[2]),
        np::RENAME => file::rename(guest, a[0], a[1]),
        np::FSYNC => file::fsync(guest, a[0]),
        np::READV => call::readv(guest, a[0], a[1], a[2]),
        np::CHMOD => file::chmod(guest, a[0], a[1]),
        np::FCHMOD => file::fchmod(guest, a[0], a[1]),
        np::FCHMODAT => file::fchmodat(guest, a[0], a[1], a[2]),
        np::FACCESSAT | np::FACCESSAT2 => file::faccessat(guest, a[0], a[1]),
        np::STATFS | np::FSTATFS => file::statfs(guest, a[1]),
        np::STATX => file::statx(guest, a[0], a[1], a[4]),
        np::EPOLL_WAIT => file::epoll_wait(guest, a[0], a[1], a[2]),
        nr::ACCESS => file::access(guest, a[0]),
        nr::READLINK => file::readlink(guest, a[0]),
        _ => return None,
    })
}
