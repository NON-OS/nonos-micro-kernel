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

use nonos_libc::{mk_time_millis, mk_yield};

use super::poll::poll_once;
use super::probe::Probe;
use super::send::send_echo;
use super::DEADLINE_MS;

const E_OK: u16 = 0;
const E_NO_CONFIG: u16 = 5;
const E_NO_ROUTE: u16 = 6;
const E_NO_NEIGHBOUR: u16 = 7;

// One bounded slice of the echo-request/echo-reply poll: send once, then
// poll for the matching reply until the per-probe deadline. Holds the
// progress cursor (send state, socket deadline) between slices.
pub struct PingJob {
    port: u32,
    dst: [u8; 4],
    t0: i64,
    seq: u16,
    sent: bool,
}

impl PingJob {
    pub fn new(port: u32, dst: [u8; 4], seq: u16) -> Self {
        Self { port, dst, t0: mk_time_millis(), seq, sent: false }
    }

    pub fn dst(&self) -> [u8; 4] {
        self.dst
    }

    pub fn step_once(&mut self) -> Option<Probe> {
        if mk_time_millis().wrapping_sub(self.t0) > DEADLINE_MS {
            return Some(if self.sent { Probe::Timeout } else { Probe::Unreachable });
        }
        if !self.sent {
            match send_echo(self.port, self.dst, self.seq) {
                E_OK => self.sent = true,
                E_NO_NEIGHBOUR => {}
                E_NO_ROUTE => return Some(Probe::NoRoute),
                E_NO_CONFIG => return Some(Probe::NotReady),
                _ => return Some(Probe::SendFailed),
            }
        }
        if let Some(rtt) = poll_once(self.port, self.dst, self.t0, self.seq) {
            return Some(Probe::Reply(rtt));
        }
        mk_yield();
        None
    }
}
