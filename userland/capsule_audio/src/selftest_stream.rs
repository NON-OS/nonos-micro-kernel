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

use nonos_libc::mk_yield;

use crate::server::pump::{self, PumpState, KEEP_AHEAD, PERIOD_SAMPLES};
use crate::server::streams::{StreamTable, E_AGAIN};
use crate::server::tone;
use crate::sink::Sink;

const TARGET_SENT: usize = 6 * KEEP_AHEAD;
const MAX_ITERS: u32 = 20_000;
const FEED_RETRIES: u32 = 100;

pub fn run_streams(sink: &Sink) {
    let mut table = StreamTable::new();
    let mut pump_state = PumpState::new();
    let id_a = match table.open(0) {
        Some(id) => id,
        None => return,
    };
    let id_b = match table.open(0) {
        Some(id) => id,
        None => return,
    };
    let mut a = [0i16; PERIOD_SAMPLES];
    let mut b = [0i16; PERIOD_SAMPLES];
    tone::synth(440, 22, 0x2000, &mut a);
    tone::synth(660, 22, 0x2000, &mut b);
    for _ in 0..MAX_ITERS {
        feed_retry(&mut table, id_a, &a);
        feed_retry(&mut table, id_b, &b);
        pump::step(&mut pump_state, &mut table, sink);
        if pump_state.sent() >= TARGET_SENT {
            break;
        }
    }
}

fn feed_retry(table: &mut StreamTable, id: u32, buf: &[i16]) {
    for _ in 0..FEED_RETRIES {
        if table.feed(id, buf) != E_AGAIN {
            return;
        }
        mk_yield();
    }
}
