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

//! `/proc/cpuinfo`.
//!
//! Split per architecture rather than shared, because the file's contents are
//! architecture specific by definition. A PC entry carries family, model,
//! stepping and a CPUID feature list; an ARM entry carries the implementer
//! and part numbers out of `MIDR_EL1` and no brand string, because the
//! hardware has none to give. Linux draws the same line, so anything that
//! already parses this file on either architecture sees the shape it expects.

extern crate alloc;

use alloc::string::String;

#[cfg(target_arch = "aarch64")]
mod aarch64;
#[cfg(target_arch = "x86_64")]
mod x86_64;

pub fn read_cpuinfo() -> String {
    #[cfg(target_arch = "x86_64")]
    {
        x86_64::read_cpuinfo()
    }
    #[cfg(target_arch = "aarch64")]
    {
        aarch64::read_cpuinfo()
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        String::new()
    }
}
