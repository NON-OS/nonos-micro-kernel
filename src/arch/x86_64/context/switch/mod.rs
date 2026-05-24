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

mod cpu_switch;
#[cfg(feature = "nonos-cpuswitch-selftest")]
mod cpu_switch_selftest;
mod dispatch;
mod first_entry;
mod kernel_thread;
mod resume;
mod trampolines;

pub(crate) use cpu_switch::build_initial_switch_frame;
pub(crate) use dispatch::switch_to_user_pcb_x86_64;
pub(crate) use trampolines::{first_entry_trampoline, resume_user_trampoline};
#[cfg(feature = "nonos-cpuswitch-selftest")]
pub(crate) use cpu_switch_selftest::run as cpu_switch_selftest;
