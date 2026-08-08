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

//! The battery-backed clock, if the board has one.
//!
//! A PC keeps it in the CMOS as a date the kernel converts to seconds; an ARM
//! board usually has a PL031, which counts seconds directly. Either way this
//! is the only source of absolute time before the network is up, and it is
//! read once at boot to anchor the cycle counter.
//!
//! `None` means there is no clock the kernel can read, which is a real answer
//! on a board that has none. Callers anchor from something else or run with an
//! unset wall clock rather than being handed a fabricated date.

/// Seconds since the Unix epoch, straight off the hardware.
pub(crate) fn unix_timestamp() -> Option<u64> {
    #[cfg(target_arch = "x86_64")]
    return Some(crate::arch::x86_64::time::rtc::read_unix_timestamp());
    #[cfg(target_arch = "aarch64")]
    return crate::arch::aarch64::rtc::unix_timestamp();
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    return None;
}

/// The same reading split into a date and time.
pub(crate) fn civil_time() -> Option<crate::sys::clock::civil::CivilTime> {
    unix_timestamp().map(|seconds| crate::sys::clock::civil::from_unix(seconds))
}
