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

use super::{RTO_INIT_MS, RTO_MAX_MS, RTO_MIN_MS};

pub struct Rtt {
    srtt: u32,
    rttvar: u32,
    rto: u32,
    has_sample: bool,
}

impl Rtt {
    pub const fn new() -> Self {
        Rtt { srtt: 0, rttvar: 0, rto: RTO_INIT_MS, has_sample: false }
    }

    pub fn rto_ms(&self) -> u32 {
        self.rto
    }

    pub fn on_sample(&mut self, r_ms: u32) {
        if !self.has_sample {
            self.srtt = r_ms;
            self.rttvar = r_ms / 2;
            self.has_sample = true;
        } else {
            let delta = if self.srtt > r_ms { self.srtt - r_ms } else { r_ms - self.srtt };
            self.rttvar = (self.rttvar * 3 + delta) / 4;
            self.srtt = (self.srtt * 7 + r_ms) / 8;
        }
        let candidate = self.srtt.saturating_add((4 * self.rttvar).max(1));
        self.rto = candidate.clamp(RTO_MIN_MS, RTO_MAX_MS);
    }

    pub fn backoff(&mut self) {
        self.rto = self.rto.saturating_mul(2).min(RTO_MAX_MS);
    }
}
