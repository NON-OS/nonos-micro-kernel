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

use super::{INIT_CWND, MSS};

pub struct Cc {
    cwnd: u32,
    ssthresh: u32,
    dupacks: u8,
}

impl Cc {
    pub const fn new() -> Self {
        Cc { cwnd: INIT_CWND, ssthresh: u32::MAX, dupacks: 0 }
    }
    pub fn cwnd(&self) -> u32 {
        self.cwnd
    }
    pub fn on_new_ack(&mut self) {
        self.dupacks = 0;
        let mss = MSS as u32;
        if self.cwnd < self.ssthresh {
            self.cwnd = self.cwnd.saturating_add(mss);
        } else {
            self.cwnd = self.cwnd.saturating_add((mss * mss) / self.cwnd.max(1));
        }
    }
    pub fn on_rto(&mut self) {
        let mss = MSS as u32;
        self.ssthresh = (self.cwnd / 2).max(2 * mss);
        self.cwnd = mss;
        self.dupacks = 0;
    }
}
