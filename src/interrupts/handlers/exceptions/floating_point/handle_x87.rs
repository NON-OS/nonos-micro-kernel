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

use x86_64::structures::idt::InterruptStackFrame;

use super::clear_x87_exception::clear_x87_exception;
use super::finish::finish_floating_point_exception;
use super::log_x87_exception::log_x87_exception;
use super::read_x87_status::read_x87_status;
use crate::interrupts::handlers::exceptions::context::ExceptionContext;
use crate::interrupts::stats;

pub fn handle_x87(frame: InterruptStackFrame) {
    let ctx = ExceptionContext::from_frame(&frame);
    stats::increment_exceptions();
    let status = read_x87_status();
    log_x87_exception(&ctx, &status);
    clear_x87_exception();
    finish_floating_point_exception(&ctx);
}
