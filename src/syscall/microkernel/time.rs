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

use super::errnos::ERRNO_FAULT;

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
    let t = match crate::arch::x86_64::time::rtc::read_rtc_checked() {
        Ok(t) => t,
        Err(_) => return -61,
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

fn clock_ready() -> bool {
    crate::sys::clock::TSC_HZ.load(Ordering::Relaxed) != 0
        && crate::sys::clock::BOOT_UNIX_MS.load(Ordering::Relaxed) != 0
}
