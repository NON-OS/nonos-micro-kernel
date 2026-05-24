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

use crate::arch::riscv64::interrupts::frame::TrapFrame;
use crate::syscall::contract::{dispatch as contract_dispatch, SyscallArgs};
use crate::syscall::numbers::SyscallNumber;
use crate::syscall::types::errnos;

pub(super) fn dispatch_ecall(frame: &mut TrapFrame) {
    let result_word = match SyscallNumber::from_u64(frame.a7 as u64) {
        Some(sc) => {
            let args = SyscallArgs::new([
                frame.a0 as u64,
                frame.a1 as u64,
                frame.a2 as u64,
                frame.a3 as u64,
                frame.a4 as u64,
                frame.a5 as u64,
            ]);
            contract_dispatch(sc, args).value as u64
        }
        None => (-(errnos::ENOSYS as i64)) as u64,
    };
    frame.a0 = result_word as usize;
    frame.advance_pc();
}
