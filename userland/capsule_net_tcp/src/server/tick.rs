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

use crate::clock::now_ms;
use crate::server::tcp_rx::drain_one;
use crate::state::{TimerKind, TABLE};

const MIN_TICK_MS: u64 = 10;
const IDLE_CAP_MS: u64 = 250;

pub fn tick() {
    let now = now_ms();
    reap_timers(now);
    crate::server::retransmit::scan(now);
    while drain_one() {}
}

fn reap_timers(now: u64) {
    let due = TABLE.lock().timers.drain_due(now);
    for (handle, kind) in due {
        match kind {
            TimerKind::TimeWait => {
                let mut t = TABLE.lock();
                t.remove_by_handle(handle);
                t.timers.cancel_all(handle);
            }
        }
    }
}

pub fn recv_budget() -> u64 {
    let t = TABLE.lock();
    if t.is_idle() {
        return 0;
    }
    match t.timers.next_deadline() {
        Some(d) => {
            let now = now_ms();
            d.saturating_sub(now).clamp(MIN_TICK_MS, IDLE_CAP_MS)
        }
        None => IDLE_CAP_MS,
    }
}
