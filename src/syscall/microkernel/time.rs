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

use core::sync::atomic::Ordering;

use super::errnos::{ERRNO_FAULT, ERRNO_INVAL};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RtcTimeAbi {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub _pad: u8,
}

pub fn sys_time_rtc(out_ptr: u64) -> i64 {
    let Some(t) = crate::arch::wall_clock::civil_time() else {
        return -61;
    };
    let abi = RtcTimeAbi {
        year: t.year,
        month: t.month,
        day: t.day,
        hour: t.hour,
        minute: t.minute,
        second: t.second,
        _pad: 0,
    };
    if crate::usercopy::validate_user_write(out_ptr, core::mem::size_of::<RtcTimeAbi>()).is_err() {
        return ERRNO_FAULT;
    }
    match crate::usercopy::write_user_value(out_ptr, &abi) {
        Ok(()) => 0,
        Err(_) => ERRNO_FAULT,
    }
}

pub fn sys_time_millis() -> i64 {
    if !clock_ready() {
        return -61;
    }
    let now = crate::sys::clock::unix_ms();
    now.min(i64::MAX as u64) as i64
}

// Monotonic milliseconds for `Instant`: the TSC-derived base clock without the
// NTP offset, so a time correction can never move `Instant` backwards the way
// it can move the wall clock `sys_time_millis` returns.
pub fn sys_time_monotonic() -> i64 {
    crate::sys::clock::base_unix_ms().min(i64::MAX as u64) as i64
}

fn clock_ready() -> bool {
    crate::sys::clock::TSC_HZ.load(Ordering::Relaxed) != 0
        && crate::sys::clock::BOOT_UNIX_MS.load(Ordering::Relaxed) != 0
}

const CLOCK_FLOOR_MS: u64 = 1_735_689_600_000;
const CLOCK_CEIL_MS: u64 = 4_102_444_800_000;

pub fn clamp_ok(correct_ms: u64) -> bool {
    correct_ms >= CLOCK_FLOOR_MS && correct_ms <= CLOCK_CEIL_MS
}

pub fn compute_offset_ms(correct_ms: u64, base_ms: u64) -> i64 {
    correct_ms as i64 - base_ms as i64
}

pub fn sys_time_adjust(correct_ms: u64) -> i64 {
    if !clamp_ok(correct_ms) {
        return ERRNO_INVAL;
    }
    let base = crate::sys::clock::base_unix_ms();
    crate::sys::clock::set_ntp_offset_ms(compute_offset_ms(correct_ms, base));
    0
}

#[cfg(test)]
mod tests {
    use super::{clamp_ok, compute_offset_ms};

    #[test]
    fn rejects_prehistoric_and_future() {
        assert!(!clamp_ok(0));
        assert!(!clamp_ok(1_600_000_000_000));
        assert!(!clamp_ok(5_000_000_000_000));
        assert!(clamp_ok(1_800_000_000_000));
    }

    #[test]
    fn offset_is_correct_minus_base() {
        assert_eq!(compute_offset_ms(1_800_000_010_000, 1_800_000_000_000), 10_000);
        assert_eq!(compute_offset_ms(1_800_000_000_000, 1_800_000_010_000), -10_000);
    }
}
