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

use nonos_libc::{mk_debug, mk_irq_ack, mk_irq_wait};

use crate::constants::{SDSTS_BCIS, SD_STS};
use crate::setup::Driver;

const PLAY_DONE: &str = "[HDA] play-complete\n";
const IRQ_WAIT_MS: u64 = 50;

pub(super) fn poll_irq(driver: &Driver, last: &mut u64, played: &mut bool) {
    let mut seq = *last;
    if mk_irq_wait(driver.handles.irq_grant_id(), *last, IRQ_WAIT_MS, &mut seq as *mut u64) >= 0
        && seq != *last
    {
        *last = seq;
        let _ = mk_irq_ack(driver.handles.irq_grant_id());
    }
    check_completion(driver, played);
}

fn check_completion(driver: &Driver, played: &mut bool) {
    let sts = unsafe { driver.regs.r8(driver.stream_off + SD_STS) };
    if sts & SDSTS_BCIS != 0 {
        unsafe { driver.regs.w8(driver.stream_off + SD_STS, SDSTS_BCIS) };
        if !*played {
            *played = true;
            mk_debug(PLAY_DONE.as_ptr(), PLAY_DONE.len());
        }
    }
}
