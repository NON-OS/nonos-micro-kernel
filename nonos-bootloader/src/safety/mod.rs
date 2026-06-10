// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

mod panic;
mod profile;
mod watchdog;

pub use panic::{
    clear_panic_state, get_panic_line, has_panicked, panic_count, record_panic, secure_halt,
    secure_reset, should_halt, PanicCategory, PanicInfo,
};
pub use watchdog::{
    check_stage_timeout, enter_stage, get_current_stage, get_stage_duration, stage_elapsed,
    BootStage, StageGuard,
};
