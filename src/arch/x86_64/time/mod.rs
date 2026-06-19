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

mod api;
mod api_init;
mod api_time;
pub mod hpet;
pub mod pit;
pub mod rtc;
pub mod timer;
pub mod tsc;

pub use api::*;
pub use hpet::{detect_hpet, read_hpet_counter};
pub use pit::{
    AccessMode as PitAccessMode, Channel as PitChannel, Mode as PitMode, PitError, PitStatistics,
    PIT_FREQUENCY,
};
pub use rtc::{
    read_rtc_time, PeriodicRate as RtcPeriodicRate, Register as RtcRegister, RtcAlarm, RtcError,
    RtcStatistics, RtcTime,
};
pub use timer::{
    cancel_timer, get_active_timer_count, get_hpet_counter, get_timer_stats, get_tsc_frequency,
    hpet_to_ns, hrtimer_after_ns, ns_to_tsc, tick, tsc_to_ns, TimerStats,
};
pub use tsc::{CalibrationSource as TscCalibrationSource, TscError, TscFeatures, TscStatistics};
