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

use nonos_libc::time::{mk_time_millis, RtcTime};

use crate::clock::civil;
use crate::clock::stopwatch::Stopwatch;
use crate::clock::tabs::Tab;
use crate::clock::timer::Timer;

pub struct State {
    pub rtc: RtcTime,
    pub now_ms: u64,
    pub tab: Tab,
    pub sw: Stopwatch,
    pub timer: Timer,
    pub edit_hour: u8,
    pub edit_min: u8,
}

impl State {
    pub fn new() -> Self {
        let mut s = State {
            rtc: RtcTime::default(),
            now_ms: 0,
            tab: Tab::Clock,
            sw: Stopwatch::default(),
            timer: Timer::default(),
            edit_hour: 0,
            edit_min: 0,
        };
        s.refresh();
        s
    }

    pub fn refresh(&mut self) {
        let n = mk_time_millis();
        if n >= 0 {
            self.now_ms = n as u64;
            let c = civil::from_unix_ms(n as u64);
            self.rtc = RtcTime {
                year: c.year,
                month: c.month,
                day: c.day,
                hour: c.hour,
                minute: c.minute,
                second: c.second,
                _pad: 0,
            };
        }
    }

    pub fn load_edit(&mut self) {
        self.edit_hour = self.rtc.hour;
        self.edit_min = self.rtc.minute;
    }
}
