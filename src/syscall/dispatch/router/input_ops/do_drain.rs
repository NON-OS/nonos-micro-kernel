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

use super::consts::{EFAULT, EINVAL, FIRST_INPUT_DRAIN, MAX_DRAIN};
use crate::kernel_core::surface_registry::{drain_input, InputEvent};
use crate::syscall::dispatch::util::errno;
use crate::syscall::SyscallResult;
use crate::usercopy::copy_to_user;

pub(super) fn do_drain(out_ptr: u64, max_events: u64) -> SyscallResult {
    if out_ptr == 0 || max_events == 0 {
        return errno(EINVAL);
    }
    let cap = core::cmp::min(max_events as usize, MAX_DRAIN);
    let mut scratch = [InputEvent::default(); MAX_DRAIN];
    let n = drain_input(&mut scratch[..cap]);
    if n == 0 {
        return SyscallResult::success_audited(0);
    }
    if option_env!("NONOS_FBCONSOLE").is_some() {
        crate::interrupts::timer::heartbeat::input_drain_blip();
    }
    let bytes = n * core::mem::size_of::<InputEvent>();
    let src = unsafe { core::slice::from_raw_parts(scratch.as_ptr() as *const u8, bytes) };
    if copy_to_user(out_ptr, src).is_err() {
        return errno(EFAULT);
    }
    crate::sys::bench::mark_once(&FIRST_INPUT_DRAIN, b"input_drain_first");
    SyscallResult::success_audited(n as i64)
}
