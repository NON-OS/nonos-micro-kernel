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

//! Syscall numbers for the path, time and process calls, transcribed from the
//! x86_64 table.

pub const CHDIR: u64 = 80;
pub const FCHDIR: u64 = 81;
pub const RENAME: u64 = 82;
pub const MKDIR: u64 = 83;
pub const RMDIR: u64 = 84;
pub const UNLINK: u64 = 87;
pub const MKDIRAT: u64 = 258;
pub const UNLINKAT: u64 = 263;
pub const TIME: u64 = 201;
pub const GETTIMEOFDAY: u64 = 96;
pub const NANOSLEEP: u64 = 35;
pub const CLOCK_NANOSLEEP: u64 = 230;
pub const GETPPID: u64 = 110;
pub const SCHED_YIELD: u64 = 24;
pub const FSYNC: u64 = 74;
pub const UMASK: u64 = 95;
pub const SETPGID: u64 = 109;
pub const GETPGRP: u64 = 111;
pub const SETSID: u64 = 112;
pub const GETPGID: u64 = 121;
pub const GETSID: u64 = 124;
pub const SETUID: u64 = 105;
pub const SETGID: u64 = 106;
pub const READV: u64 = 19;
pub const GETRLIMIT: u64 = 97;
pub const SETRLIMIT: u64 = 160;
pub const PRLIMIT64: u64 = 302;
pub const SELECT: u64 = 23;
pub const PSELECT6: u64 = 270;
pub const CHMOD: u64 = 90;
pub const FCHMOD: u64 = 91;
pub const FCHMODAT: u64 = 268;
pub const FACCESSAT: u64 = 269;
pub const FACCESSAT2: u64 = 439;
pub const STATFS: u64 = 137;
pub const FSTATFS: u64 = 138;
pub const STATX: u64 = 332;
pub const KILL: u64 = 62;
pub const TKILL: u64 = 200;
pub const GETRESUID: u64 = 118;
pub const GETRESGID: u64 = 120;
pub const PPOLL: u64 = 271;
pub const EPOLL_WAIT: u64 = 232;
