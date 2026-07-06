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

use nonos_libc::time::{mk_time_rtc, RtcTime};

use crate::clock::tabs::Tab;

pub struct State {
    pub rtc: RtcTime,
    pub tab: Tab,
}

impl State {
    pub fn new() -> Self {
        let mut s = State {
            rtc: RtcTime::default(),
            tab: Tab::Clock,
        };
        s.refresh();
        s
    }

    pub fn refresh(&mut self) {
        let mut t = RtcTime::default();
        if mk_time_rtc(&mut t as *mut RtcTime) == 0 {
            self.rtc = t;
        }
    }
}
