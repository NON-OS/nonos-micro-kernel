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

use super::status::X87Status;
use crate::interrupts::handlers::exceptions::context::ExceptionContext;

pub fn log_x87_exception(ctx: &ExceptionContext, status: &X87Status) {
    crate::log::logger::log_warning!(
        "x87 FP Exception at {:#x}: IE={} DE={} ZE={} OE={} UE={} PE={} SF={}",
        ctx.instruction_pointer,
        (status.bits & 0x01) != 0,
        (status.bits & 0x02) != 0,
        (status.bits & 0x04) != 0,
        (status.bits & 0x08) != 0,
        (status.bits & 0x10) != 0,
        (status.bits & 0x20) != 0,
        (status.bits & 0x40) != 0
    );
}
