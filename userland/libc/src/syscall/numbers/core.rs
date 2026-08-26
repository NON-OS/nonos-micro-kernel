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
use super::tag::tag4;

pub(crate) const N_MK_MMAP: i64 = tag4(b"MMAP");
pub(crate) const N_MK_CAPSULE_LOAD: i64 = tag4(b"MCLD");
pub(crate) const N_MK_CAPSULE_VERIFY: i64 = tag4(b"MCVF");
pub(crate) const N_MK_EXIT: i64 = tag4(b"MEXT");
pub(crate) const N_MK_PID_ALIVE: i64 = tag4(b"MPAL");
pub(crate) const N_MK_GETPID: i64 = tag4(b"MGPD");
pub(crate) const N_MK_ARGS: i64 = tag4(b"MKAR");
pub(crate) const N_MK_YIELD: i64 = tag4(b"MYLD");
pub(crate) const N_MK_SLEEP_MS: i64 = tag4(b"MSLP");
pub(crate) const N_MK_TIME_MILLIS: i64 = tag4(b"MTMS");
/// The monotonic clock. There is no "MUPT" syscall in the kernel and never
/// was: a call to it returned ENOSYS, so every bound built on this wrapper
/// compared a constant negative against a deadline and never fired.
pub(crate) const N_MK_TIME_MONOTONIC: i64 = tag4(b"MMON");
pub(crate) const N_MK_TIME_ADJUST: i64 = tag4(b"MTAD");
pub(crate) const N_MK_TIME_RTC: i64 = tag4(b"MTRT");
pub(crate) const N_MK_BATTERY_STATUS: i64 = tag4(b"MBAT");
pub(crate) const N_MK_PROC_STAT: i64 = tag4(b"MPST");
pub(crate) const N_MK_PROC_OUTPUT: i64 = tag4(b"MOUT");
pub(crate) const N_MK_ATTEST_STATUS: i64 = tag4(b"MAST");
pub(crate) const N_MK_SPAWN_INSTANCE: i64 = tag4(b"MSPI");
pub(crate) const N_MK_TOOL_RUN: i64 = tag4(b"MTRN");
pub(crate) const N_MK_WAIT: i64 = tag4(b"MWAT");
pub(crate) const N_MK_KILL: i64 = tag4(b"MKIL");
pub(crate) const N_MK_PROC_INPUT: i64 = tag4(b"MPIN");
pub(crate) const N_MK_STDIN_READ: i64 = tag4(b"MSRD");
