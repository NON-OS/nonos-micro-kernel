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

use super::consts::ENOTSUP;
use super::{do_drain::do_drain, do_post::do_post, do_wait::do_wait};
use crate::syscall::dispatch::util::errno;
use crate::syscall::numbers::SyscallNumber;
use crate::syscall::SyscallResult;

pub(in crate::syscall::dispatch::router) fn handle(
    nr: SyscallNumber,
    a0: u64,
    a1: u64,
    a2: u64,
    _a3: u64,
    _a4: u64,
    _a5: u64,
) -> SyscallResult {
    match nr {
        SyscallNumber::MkInputEventPost => do_post(a0),
        SyscallNumber::MkInputEventDrain => do_drain(a0, a1),
        SyscallNumber::MkInputEventWait => do_wait(a0, a1, a2),
        _ => errno(ENOTSUP),
    }
}
