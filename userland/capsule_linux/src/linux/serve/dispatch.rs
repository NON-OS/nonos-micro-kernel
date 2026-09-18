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

//! One refused call, answered. Every number a guest can ask for arrives
//! here; the ones with no handler yet are named on the log and refused
//! with `ENOSYS`, so what is missing is a list and never a guess.

use nonos_libc::ForeignFrame;

use crate::linux::abi::{errno, nr};
use crate::linux::call;
use crate::linux::file;
use crate::linux::file::flags;
use crate::linux::guest::Guest;

pub fn answer(guest: &mut Guest, frame: &ForeignFrame) -> u64 {
    let a = frame.args();
    match frame.nr {
        nr::WRITE => call::write(guest, a[0], a[1], a[2]),
        nr::WRITEV => call::writev(guest, a[0], a[1], a[2]),
        nr::READ => call::read(guest, a[0], a[1], a[2]),
        nr::CLOSE => call::close(guest, a[0]),
        nr::OPENAT => file::openat(guest, a[0], a[1], a[2]),
        nr::OPEN => file::openat(guest, flags::AT_FDCWD, a[0], a[1]),
        nr::LSEEK => file::lseek(guest, a[0], a[1], a[2]),
        nr::FSTAT => file::fstat(guest, a[0], a[1]),
        nr::STAT | nr::LSTAT => file::newfstatat(guest, flags::AT_FDCWD, a[0], a[1]),
        nr::NEWFSTATAT => file::newfstatat(guest, a[0], a[1], a[2]),
        nr::GETDENTS64 => file::getdents64(guest, a[0], a[1], a[2]),
        nr::PREAD64 => file::pread64(guest, a[0], a[1], a[2], a[3]),
        nr::GETCWD => file::getcwd(guest, a[0], a[1]),
        nr::ACCESS => file::access(guest, a[0]),
        nr::READLINK => file::readlink(guest, a[0]),
        nr::IOCTL => call::ioctl(guest, a[0], a[1]),
        nr::FCNTL => call::fcntl(guest, a[0], a[1]),
        nr::UNAME => call::uname(guest, a[0]),
        nr::BRK => call::brk(guest, a[0]),
        nr::MMAP => call::mmap(guest, call::MapReq::from_args(a)),
        nr::MUNMAP => call::munmap(guest, a[0], a[1]),
        nr::MPROTECT => call::mprotect(guest, a[0], a[1], a[2]),
        nr::MADVISE | nr::RSEQ | nr::SET_ROBUST_LIST => errno::ok(0),
        nr::ARCH_PRCTL => call::arch_prctl(guest, a[0], a[1]),
        nr::SET_TID_ADDRESS | nr::GETTID | nr::GETPID => errno::ok(guest.pid as u64),
        nr::GETUID | nr::GETEUID | nr::GETGID | nr::GETEGID => errno::ok(0),
        nr::CLOCK_GETTIME => call::clock_gettime(guest, a[0], a[1]),
        nr::GETRANDOM => call::getrandom(guest, a[0], a[1], a[2]),
        nr::EXIT | nr::EXIT_GROUP => call::exit(guest, a[0]),
        other => super::unserved::unserved(other),
    }
}
