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

pub mod clint;

mod convert;
mod deadline;
mod delay;
mod frequency;
mod interrupt;
mod read;

pub use clint::{clear_ipi, clear_timer_interrupt, is_ipi_pending, read_mtime, send_ipi, set_clint_base, set_timer_interrupt, Clint};
pub use convert::{current_time_ns, current_time_us, ns_to_ticks, ticks_to_ns};
pub use deadline::{init_timer, init_timer_hart, set_next_timer};
pub use delay::{delay_ms, delay_ns, delay_us};
pub use frequency::set_frequency;
pub use interrupt::handle_timer_interrupt;
pub use read::read_time;
