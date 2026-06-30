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

use x86_64::registers::control::{Cr0, Cr0Flags};

pub fn handle_task_switch_fpu() {
    unsafe {
        Cr0::update(|cr0| {
            cr0.remove(Cr0Flags::TASK_SWITCHED);
        });
    }
    crate::log::logger::log_debug!("FPU context restored after task switch");
}
