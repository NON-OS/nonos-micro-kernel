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

//! The per-thread state a C runtime sets up before it will run any code
//! of its own: the thread pointer, the clock, and randomness.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

const ARCH_SET_FS: u64 = 0x1002;
const ARCH_GET_FS: u64 = 0x1003;

/// The thread pointer belongs to the calling thread, not the process, so the
/// tid is the one that traps and not the guest's own pid.
pub fn arch_prctl(guest: &mut Guest, tid: u32, code: u64, addr: u64) -> u64 {
    match code {
        ARCH_SET_FS => {
            if nonos_libc::peer::mk_peer_tls(tid, addr) < 0 {
                return errno::fail(errno::EPERM);
            }
            guest.fs_base = addr;
            errno::ok(0)
        }
        ARCH_GET_FS => match guest.write(addr, &guest.fs_base.to_le_bytes()) {
            n if n < 0 => errno::fail(errno::EFAULT),
            _ => errno::ok(0),
        },
        _ => errno::fail(errno::EINVAL),
    }
}

/// Seconds and nanoseconds, from the host's own monotonic millisecond clock.
pub fn clock_gettime(guest: &mut Guest, _clock: u64, out: u64) -> u64 {
    let ms = nonos_libc::mk_uptime_ms().max(0) as u64;
    let mut buf = [0u8; 16];
    buf[..8].copy_from_slice(&(ms / 1000).to_le_bytes());
    buf[8..].copy_from_slice(&((ms % 1000) * 1_000_000).to_le_bytes());
    if guest.write(out, &buf) < 0 {
        return errno::fail(errno::EFAULT);
    }
    errno::ok(0)
}

/// Randomness from the kernel's own source, so a guest's keys are as good
/// as a capsule's.
pub fn getrandom(guest: &mut Guest, buf: u64, len: u64, _flags: u64) -> u64 {
    let take = len.min(256) as usize;
    let mut bytes = alloc::vec![0u8; take];
    if nonos_libc::crypto_random(bytes.as_mut_ptr(), take) < 0 {
        return errno::fail(errno::EAGAIN);
    }
    if guest.write(buf, &bytes) < 0 {
        return errno::fail(errno::EFAULT);
    }
    errno::ok(take as u64)
}
