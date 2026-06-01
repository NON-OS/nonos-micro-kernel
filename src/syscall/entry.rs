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

use super::contract::{dispatch as contract_dispatch, SyscallArgs};
use super::numbers::SyscallNumber;
use super::types::errnos;

fn ret_errno(e: i32) -> u64 {
    (-(e as i64)) as u64
}

pub fn handle_syscall(id: u64, a0: u64, a1: u64, a2: u64, a3: u64, a4: u64, a5: u64) -> u64 {
    let Some(number) = SyscallNumber::from_u64(id) else {
        return ret_errno(errnos::ENOSYS);
    };
    contract_dispatch(number, SyscallArgs::new([a0, a1, a2, a3, a4, a5])).value as u64
}
