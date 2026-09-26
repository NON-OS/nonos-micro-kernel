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

//! Names for the numbers, so a guest that asks for something not yet served is
//! reported by name rather than by integer.

use super::nr;

pub fn of(number: u64) -> &'static [u8] {
    match number {
        nr::READ => b"read",
        nr::WRITE => b"write",
        nr::OPEN => b"open",
        nr::CLOSE => b"close",
        nr::STAT => b"stat",
        nr::FSTAT => b"fstat",
        nr::LSTAT => b"lstat",
        nr::POLL => b"poll",
        nr::LSEEK => b"lseek",
        nr::MMAP => b"mmap",
        nr::MPROTECT => b"mprotect",
        nr::MUNMAP => b"munmap",
        nr::BRK => b"brk",
        nr::RT_SIGACTION => b"rt_sigaction",
        nr::RT_SIGPROCMASK => b"rt_sigprocmask",
        nr::IOCTL => b"ioctl",
        nr::READV => b"readv",
        nr::WRITEV => b"writev",
        nr::ACCESS => b"access",
        nr::MADVISE => b"madvise",
        nr::NANOSLEEP => b"nanosleep",
        nr::GETPID => b"getpid",
        nr::EXIT => b"exit",
        nr::UNAME => b"uname",
        nr::FCNTL => b"fcntl",
        nr::GETCWD => b"getcwd",
        nr::READLINK => b"readlink",
        nr::SIGALTSTACK => b"sigaltstack",
        nr::ARCH_PRCTL => b"arch_prctl",
        nr::GETTID => b"gettid",
        nr::FUTEX => b"futex",
        nr::SET_TID_ADDRESS => b"set_tid_address",
        nr::CLOCK_GETTIME => b"clock_gettime",
        nr::EXIT_GROUP => b"exit_group",
        nr::CLONE => b"clone",
        nr::SOCKET => b"socket",
        nr::SENDMSG => b"sendmsg",
        nr::RECVMSG => b"recvmsg",
        nr::MEMFD_CREATE => b"memfd_create",
        nr::CONNECT => b"connect",
        nr::GETDENTS64 => b"getdents64",
        nr::OPENAT => b"openat",
        nr::NEWFSTATAT => b"newfstatat",
        nr::SET_ROBUST_LIST => b"set_robust_list",
        nr::PRLIMIT64 => b"prlimit64",
        nr::GETRANDOM => b"getrandom",
        nr::RSEQ => b"rseq",
        _ => b"?",
    }
}
