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

mod state;
mod switch;
mod syscall_rsp;
mod tick;
mod yield_body;
mod yield_impl;

pub(crate) use state::SCHEDULER_STATS;
pub(crate) use syscall_rsp::save_syscall_user_rsp;
pub use state::{clear_reschedule, need_reschedule};
pub use state::{CURRENT_TIME_SLICE, DEFAULT_TIME_SLICE, NEED_RESCHEDULE};
pub(crate) use switch::preempt_current_process;
pub use tick::tick;
pub(crate) use yield_body::perform_yield_inline;
pub use yield_impl::yield_now;
