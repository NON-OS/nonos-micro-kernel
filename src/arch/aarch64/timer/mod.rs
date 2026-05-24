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

mod clock;
mod control;
mod deadline;
mod delay;
pub mod generic;
mod init;
mod interrupt;
pub mod physical;
pub mod preemption;
mod state;
pub mod virtual_timer;

pub use clock::{current_time_ns, current_time_us};
pub use deadline::{disable_timer, set_timer};
pub use delay::{delay_ms, delay_ns, delay_us};
pub use generic::{current_count, frequency, nanoseconds_to_ticks, ticks_to_nanoseconds};
pub use init::{init_timer, init_timer_cpu};
pub use interrupt::handle_timer_interrupt;
pub use physical::{set_physical_timer, PhysicalTimer};
pub use preemption::{configure as configure_preemption_intid, install_on_cpu};
pub use virtual_timer::{set_virtual_timer, VirtualTimer};
