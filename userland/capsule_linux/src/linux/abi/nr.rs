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

//! Linux x86_64 syscall numbers, by family. These are Linux's numbers and
//! never this system's: they are the contract a compiled binary was built
//! against, so they are transcribed rather than chosen.

pub const READ: u64 = 0;
pub const WRITE: u64 = 1;
pub const OPEN: u64 = 2;
pub const CLOSE: u64 = 3;
pub const STAT: u64 = 4;
pub const FSTAT: u64 = 5;
pub const LSTAT: u64 = 6;
pub const POLL: u64 = 7;
pub const LSEEK: u64 = 8;
pub const MMAP: u64 = 9;
pub const MPROTECT: u64 = 10;
pub const MUNMAP: u64 = 11;
pub const BRK: u64 = 12;
pub const RT_SIGACTION: u64 = 13;
pub const RT_SIGPROCMASK: u64 = 14;
pub const IOCTL: u64 = 16;
pub const PREAD64: u64 = 17;
pub const PWRITE64: u64 = 18;
pub const READV: u64 = 19;
pub const WRITEV: u64 = 20;
pub const ACCESS: u64 = 21;
pub const MADVISE: u64 = 28;
pub const SOCKET: u64 = 41;
pub const CONNECT: u64 = 42;
pub const SENDTO: u64 = 44;
pub const RECVFROM: u64 = 45;
pub const SHUTDOWN: u64 = 48;
pub const NANOSLEEP: u64 = 35;
pub const GETPID: u64 = 39;
pub const CLONE: u64 = 56;
pub const EXIT: u64 = 60;
pub const UNAME: u64 = 63;
pub const FCNTL: u64 = 72;
pub const GETCWD: u64 = 79;
pub const READLINK: u64 = 89;
pub const GETUID: u64 = 102;
pub const GETGID: u64 = 104;
pub const GETEUID: u64 = 107;
pub const GETEGID: u64 = 108;
pub const SIGALTSTACK: u64 = 131;
pub const ARCH_PRCTL: u64 = 158;
pub const GETTID: u64 = 186;
pub const FUTEX: u64 = 202;
pub const GETDENTS64: u64 = 217;
pub const SET_TID_ADDRESS: u64 = 218;
pub const CLOCK_GETTIME: u64 = 228;
pub const EXIT_GROUP: u64 = 231;
pub const OPENAT: u64 = 257;
pub const NEWFSTATAT: u64 = 262;
pub const SET_ROBUST_LIST: u64 = 273;
pub const PRLIMIT64: u64 = 302;
pub const GETRANDOM: u64 = 318;
pub const RSEQ: u64 = 334;
