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

use super::consts::{EFAULT, ENOMEM};
use crate::kernel_core::surface_registry::{post_input, InputEvent};
use crate::syscall::dispatch::util::errno;
use crate::syscall::SyscallResult;
use crate::usercopy::read_user_value;

pub(super) fn do_post(ev_ptr: u64) -> SyscallResult {
    let ev: InputEvent = match read_user_value(ev_ptr) {
        Ok(v) => v,
        Err(_) => return errno(EFAULT),
    };
    if option_env!("NONOS_FBCONSOLE").is_some() {
        crate::interrupts::timer::heartbeat::input_post_blip();
    }
    match post_input(ev) {
        Ok(()) => SyscallResult::success_audited(0),
        Err(_) => errno(ENOMEM),
    }
}
