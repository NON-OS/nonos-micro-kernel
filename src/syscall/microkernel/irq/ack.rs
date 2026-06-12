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

use crate::hardware::broker::IrqError;
use crate::process::current_pid;
use crate::syscall::microkernel::errnos::{ERRNO_INVAL, ERRNO_NODEV, ERRNO_PERM};
use core::sync::atomic::{AtomicU32, Ordering};

static IRQ_ACK_TRACE_COUNT: AtomicU32 = AtomicU32::new(0);

fn trace(label: &[u8], pid: u32) {
    if !matches!(pid, 7 | 8 | 0x1c) || IRQ_ACK_TRACE_COUNT.fetch_add(1, Ordering::Relaxed) >= 32 {
        return;
    }
    crate::sys::serial::print(b"[IRQ-ACK] ");
    crate::sys::serial::println(label);
}

// Only the bound owner may ack; cross-pid ack is rejected by the
// broker's grant-ownership check.
pub fn sys_irq_ack(grant_id: u64) -> i64 {
    let pid = match current_pid() {
        Some(p) => p,
        None => return ERRNO_PERM,
    };
    trace(b"enter", pid);
    match crate::hardware::broker::irq_ack_grant(pid, grant_id) {
        Ok(()) => {
            trace(b"ok", pid);
            0
        }
        Err(IrqError::NotHolder) => ERRNO_PERM,
        Err(IrqError::UnknownGrant) => ERRNO_INVAL,
        Err(IrqError::PlatformError) => ERRNO_NODEV,
    }
}
