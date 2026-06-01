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

use super::raw::raw;

/// Run a syscall and return the kernel value verbatim. Negative
/// returns are the negated kernel errno; the caller decides how to
/// surface that. The POSIX `-1 + errno` fold is a separate bridge that
/// arrives with per-thread errno storage.
#[inline]
pub fn call_raw(num: i64, args: [u64; 6]) -> i64 {
    // SAFETY: the SYSCALL instruction itself is the only unsafe
    // operation; the kernel validates argument semantics and returns
    // -EFAULT on a bad pointer rather than producing UB.
    unsafe { raw(num, args[0], args[1], args[2], args[3], args[4], args[5]) }
}

/// Run a syscall whose handler must not return (`MkExit`). If the
/// kernel ever returns, the thread parks rather than continuing in
/// undefined state.
#[inline]
pub(crate) fn call_diverging(num: i64, args: [u64; 6]) -> ! {
    // SAFETY: same as `call_raw`; this variant additionally asserts
    // divergence by parking on the impossible return path.
    let _ = unsafe { raw(num, args[0], args[1], args[2], args[3], args[4], args[5]) };
    loop {
        core::hint::spin_loop();
    }
}
