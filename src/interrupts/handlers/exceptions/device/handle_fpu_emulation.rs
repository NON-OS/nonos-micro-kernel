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

use crate::interrupts::handlers::exceptions::context::ExceptionContext;
use crate::interrupts::idt::halt_loop;

pub(crate) fn handle_fpu_emulation(ctx: &ExceptionContext) -> ! {
    crate::log::logger::log_warning!("FPU emulation not supported");
    if ctx.is_user_mode() {
        crate::process::exit::exit_and_yield(-4, true)
    }
    halt_loop();
}
