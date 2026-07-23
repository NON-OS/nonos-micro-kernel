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

use nonos_libc::mk_debug;

use crate::audio::PcmQueue;
use crate::constants::SD_LPIB;
use crate::controller::bdl::{N_PERIODS, PERIOD_BYTES};
use crate::setup::Driver;

const REFILL_MARK: &str = "[HDA] refill\n";
const UNDERRUN_MARK: &str = "[HDA] underrun\n";
const MARK_CAP: u32 = 64;

pub(super) struct Refill {
    wpos: usize,
    marks: u32,
    unders: u32,
}

impl Refill {
    pub(super) fn new() -> Self {
        Refill { wpos: 0, marks: 0, unders: 0 }
    }

    pub(super) fn reset(&mut self) {
        self.wpos = 0;
        self.marks = 0;
        self.unders = 0;
    }
}

pub(super) fn refill(driver: &Driver, q: &mut PcmQueue, rf: &mut Refill) {
    let lpib = unsafe { driver.regs.r32(driver.stream_off + SD_LPIB) } as u64;
    let rp = ((lpib / PERIOD_BYTES) as usize) % N_PERIODS;
    while rf.wpos != rp {
        let base = driver.sample.user_va + (rf.wpos as u64) * PERIOD_BYTES;
        let slot = unsafe { core::slice::from_raw_parts_mut(base as *mut u8, PERIOD_BYTES as usize) };
        if q.pop_into(slot) == PERIOD_BYTES as usize {
            emit(&mut rf.marks, REFILL_MARK);
        } else {
            emit(&mut rf.unders, UNDERRUN_MARK);
        }
        rf.wpos = (rf.wpos + 1) % N_PERIODS;
    }
}

fn emit(counter: &mut u32, s: &str) {
    if *counter < MARK_CAP {
        *counter += 1;
        mk_debug(s.as_ptr(), s.len());
    }
}
