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

mod consts;
mod convert;
mod init;
mod rdtsc;

pub use consts::{BOOT_EPOCH_MS, BOOT_TSC, TIMER_INIT, TSC_FREQ_HZ};
pub use convert::{ms_to_ticks, ticks_to_ms, ticks_to_ns, ticks_to_us, tsc_frequency, us_to_ticks};
pub use init::{calibrate_tsc_hz, init, init_default};
pub use rdtsc::rdtsc;
