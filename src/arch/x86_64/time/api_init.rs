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

mod accept;
mod stats;

use super::{hpet, pit, rtc, timer as nonos_timer, tsc};
use super::{RtcError, RtcTime};
use self::accept::{accept_pit, accept_rtc, accept_tsc};
pub use self::stats::get_all_stats;

#[inline(always)]
pub fn rdtsc() -> u64 {
    tsc::rdtsc()
}

#[inline(always)]
pub fn tsc_now() -> u64 {
    tsc::rdtsc()
}

#[inline(always)]
pub fn rdtscp() -> (u64, u32) {
    tsc::rdtscp()
}

#[inline(always)]
pub fn read_rtc() -> RtcTime {
    rtc::read_rtc()
}

#[inline(always)]
pub fn read_rtc_checked() -> Result<RtcTime, RtcError> {
    rtc::read_rtc_checked()
}

#[inline(always)]
pub fn unix_timestamp() -> u64 {
    rtc::read_unix_timestamp()
}

pub fn init() -> Result<(), &'static str> {
    accept_tsc(tsc::init())?;
    accept_pit(pit::init())?;
    accept_rtc(rtc::init())?;
    nonos_timer::init();
    Ok(())
}

pub fn init_with_hpet(hpet_base: u64) -> Result<(), &'static str> {
    if hpet_base == 0 || hpet::detect_hpet() != Some(hpet_base) {
        accept_tsc(tsc::init())?;
    } else {
        accept_tsc(tsc::init_with_hpet(hpet_base))?;
    }
    accept_pit(pit::init())?;
    accept_rtc(rtc::init())?;
    nonos_timer::init();
    Ok(())
}
