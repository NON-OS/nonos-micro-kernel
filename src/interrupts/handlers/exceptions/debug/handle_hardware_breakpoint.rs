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

use super::types::DebugInfo;
use crate::interrupts::handlers::exceptions::context::ExceptionContext;

pub(crate) fn handle_hardware_breakpoint(ctx: &ExceptionContext, info: &DebugInfo) {
    let bp_num = if info.breakpoint_0 {
        0
    } else if info.breakpoint_1 {
        1
    } else if info.breakpoint_2 {
        2
    } else {
        3
    };
    crate::log::logger::log_debug!(
        "Hardware breakpoint {} at rip={:#x}",
        bp_num,
        ctx.instruction_pointer
    );
}
