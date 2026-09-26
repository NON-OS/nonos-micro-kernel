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


//! Signal dispositions, recorded and never delivered.
//!
//! Delivery means pushing a frame onto a guest thread's stack and
//! redirecting it, which needs the guest's register state, and the trap
//! mechanism hands out a frame but no way to rewrite one. So the
//! handlers a program installs are remembered and nothing is ever
//! raised. That is a real limit and it is recorded here rather than
//! hidden behind a success: a program whose correctness depends on
//! SIGALRM firing will hang, not misbehave quietly.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

/// Linux refuses to let these two be caught, and so does this.
const SIGKILL: u64 = 9;
const SIGSTOP: u64 = 19;

/// The largest signal number Linux defines.
const NSIG: u64 = 64;

pub fn rt_sigaction(guest: &mut Guest, signum: u64, act: u64, old: u64) -> u64 {
    if signum == 0 || signum > NSIG || signum == SIGKILL || signum == SIGSTOP {
        return errno::fail(errno::EINVAL);
    }
    if old != 0 && guest.write(old, &[0u8; SIGACTION_LEN]) < SIGACTION_LEN as i64 {
        return errno::fail(errno::EFAULT);
    }
    if act != 0 {
        guest.handlers[signum as usize - 1] = true;
    }
    errno::ok(0)
}

/// `struct sigaction` on x86_64: handler, flags, restorer, mask.
const SIGACTION_LEN: usize = 32;

/// The mask is recorded nowhere because nothing is ever raised against
/// it. Reporting an empty old mask is true: no signal is pending.
pub fn rt_sigprocmask(guest: &Guest, old: u64) -> u64 {
    if old != 0 && guest.write(old, &[0u8; 8]) < 8 {
        return errno::fail(errno::EFAULT);
    }
    errno::ok(0)
}

/// An alternate stack for a handler that will never run.
pub fn sigaltstack(guest: &Guest, old: u64) -> u64 {
    if old != 0 && guest.write(old, &[0u8; 24]) < 24 {
        return errno::fail(errno::EFAULT);
    }
    errno::ok(0)
}
