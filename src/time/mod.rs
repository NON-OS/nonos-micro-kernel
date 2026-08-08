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

//! How long this boot has been running.
//!
//! Sixty-seven files ask the kernel what time it is, and until now the answer
//! came from `arch::x86_64::time` re-exported as `crate::time`, so the whole
//! kernel's notion of elapsed time was a PC's. Underneath it was only ever a
//! cycle counter, an anchor and a division, and both architectures have the
//! counter: `x86_64` a TSC, `aarch64` `CNTPCT_EL0` with its rate in
//! `CNTFRQ_EL0`. So the division moved here and the architecture keeps only
//! the register read.
//!
//! This is elapsed time, not the date. Anything that needs to know the year
//! wants `arch::wall_clock`, which reads the RTC.

mod boot;
mod now;
mod units;

pub(crate) use boot::anchor;
pub use now::now_ns;
pub use units::{
    current_ticks, get_kernel_time_ns, monotonic_ns, timestamp_millis, timestamp_secs,
};
