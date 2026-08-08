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

#[cfg(target_arch = "x86_64")]
use crate::process::nonos_core::PROCESS_TABLE;
#[cfg(target_arch = "x86_64")]
use core::sync::atomic::Ordering;

#[cfg(target_arch = "x86_64")]
pub(crate) fn save_syscall_user_rsp(pid: u32) {
    let rsp: u64;
    unsafe {
        core::arch::asm!(
            "mov {0}, gs:[{off}]",
            out(reg) rsp,
            off = const crate::smp::percpu::layout::USER_STACK_SAVED,
            options(nomem, nostack, preserves_flags),
        );
    }
    if let Some(pcb) = PROCESS_TABLE.find_by_pid(pid) {
        pcb.syscall_user_rsp.store(rsp, Ordering::Relaxed);
    }
}

#[cfg(not(target_arch = "x86_64"))]
pub(crate) fn save_syscall_user_rsp(_pid: u32) {}
