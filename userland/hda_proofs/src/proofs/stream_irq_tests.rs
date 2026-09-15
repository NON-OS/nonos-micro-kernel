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

//! Arming the interrupt that keeps the ring fed.
//!
//! Playback is a loop with the buffer completion interrupt at its top: the
//! engine finishes a period, raises it, and the capsule refills behind the
//! read pointer. Every gate on that path has to be open at once, the global
//! enable, this stream's own bit and the descriptor's own enable, and each of
//! them is silently optional as far as the hardware is concerned. With any one
//! shut the first four periods play and then the ring repeats forever.

use nonos_devmodel::FakeBar;

use crate::constants::{INTCTL, INTCTL_GIE, SDCTL_IOCE, SDCTL_RUN, SDSTS_BCIS, SD_CTL, SD_STS};
use crate::model::WINDOW;
use crate::proofs::stream_tests::{play, BDL_BYTES, INDEX, OFF};

/// A bit belonging to another stream, of the kind an earlier capsule leaves
/// behind in this register.
const OTHER_STREAM: u32 = 1 << 7;

#[test]
fn the_engine_is_left_running_with_its_completion_interrupt_enabled() {
    let (bar, bdl) = (FakeBar::new(WINDOW), FakeBar::new(BDL_BYTES));
    play(&bar, &bdl);
    assert_eq!(bar.wrote8((OFF + SD_CTL) as usize), SDCTL_RUN | SDCTL_IOCE);
}

#[test]
fn a_stale_completion_status_is_cleared_before_the_engine_is_started() {
    /*
     * The status bit latches and is cleared by writing a one back. Left set
     * from a previous run, the first real completion is indistinguishable from
     * it and the refill either fires early or is skipped entirely.
     */
    let (bar, bdl) = (FakeBar::new(WINDOW), FakeBar::new(BDL_BYTES));
    bar.present8((OFF + SD_STS) as usize, SDSTS_BCIS);
    play(&bar, &bdl);
    assert_eq!(bar.wrote8((OFF + SD_STS) as usize), SDSTS_BCIS, "the latch was not acknowledged");
}

#[test]
fn enabling_this_stream_leaves_the_interrupts_already_enabled_alone() {
    /*
     * This register carries one bit per stream plus the global enable, so it
     * is shared state. A blind write of the global enable and this stream's
     * bit switches off every other stream's interrupt, which stops their
     * refill loops without stopping their engines.
     */
    let (bar, bdl) = (FakeBar::new(WINDOW), FakeBar::new(BDL_BYTES));
    bar.present32(INTCTL as usize, OTHER_STREAM);
    play(&bar, &bdl);
    let want = OTHER_STREAM | INTCTL_GIE | (1 << INDEX);
    assert_eq!(bar.wrote32(INTCTL as usize), want, "another stream's interrupt was disabled");
}
