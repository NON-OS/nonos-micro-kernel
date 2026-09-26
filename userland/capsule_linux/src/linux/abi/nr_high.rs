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


//! Linux x86_64 syscall numbers from one hundred up. Same contract
//! as `nr`, split only because a file here stays under seventy-five lines.

pub const GETUID: u64 = 102;
pub const GETGID: u64 = 104;
pub const GETEUID: u64 = 107;
pub const GETEGID: u64 = 108;
pub const SIGALTSTACK: u64 = 131;
pub const ARCH_PRCTL: u64 = 158;
pub const GETTID: u64 = 186;
pub const FUTEX: u64 = 202;
pub const GETDENTS64: u64 = 217;
pub const WAIT4: u64 = 61;
pub const EPOLL_CTL: u64 = 233;
pub const DUP3: u64 = 292;
pub const PIPE2: u64 = 293;
pub const TIMERFD_CREATE: u64 = 283;
pub const TIMERFD_SETTIME: u64 = 286;
pub const EPOLL_CREATE1: u64 = 291;
pub const EPOLL_PWAIT: u64 = 281;
pub const SET_TID_ADDRESS: u64 = 218;
pub const CLOCK_GETTIME: u64 = 228;
pub const EXIT_GROUP: u64 = 231;
pub const OPENAT: u64 = 257;
pub const NEWFSTATAT: u64 = 262;
pub const SET_ROBUST_LIST: u64 = 273;
pub const PRLIMIT64: u64 = 302;
pub const GETRANDOM: u64 = 318;
pub const MEMFD_CREATE: u64 = 319;
pub const RSEQ: u64 = 334;
