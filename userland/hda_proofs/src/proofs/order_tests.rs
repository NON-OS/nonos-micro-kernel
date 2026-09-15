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

//! What the stream engine can see at the moment it is switched on.
//!
//! Setting RUN hands the buffer registers to a bus master, and the driver gets
//! no second chance to say where they point: the engine is already reading. A
//! run bit set ahead of the address registers is a DMA read from whatever the
//! register happened to hold, which after a warm reboot is the previous
//! system's ring and after a cold one is physical page zero.

use std::sync::Arc;

use nonos_devmodel::{run, FakeBar};

use crate::constants::{SDCTL_RUN, SD_BDPL, SD_BDPU, SD_CBL, SD_CTL, SD_LVI};
use crate::controller::bdl::{N_PERIODS, RING_BYTES};
use crate::model::WINDOW;
use crate::proofs::at_start::{wait_for, watch, AtStart};
use crate::proofs::stream_tests::{play, BDL_BYTES, BDL_DEV, OFF};

#[test]
fn the_stream_engine_is_pointed_at_its_buffer_before_it_is_told_to_run() {
    let bar = Arc::new(FakeBar::new(WINDOW));
    let bdl = FakeBar::new(BDL_BYTES);
    let s = Arc::new(AtStart::default());
    let ctl = (OFF + SD_CTL) as usize;
    let at = [SD_CBL, SD_BDPL, SD_BDPU, SD_LVI].map(|r| (OFF + r) as usize);
    let device = run(&bar, watch(Arc::clone(&s), ctl, SDCTL_RUN, at));
    play(&bar, &bdl);
    wait_for(&s);
    drop(device);
    assert_eq!(s.at(0), RING_BYTES as u32, "started before the ring length was set");
    assert_eq!(s.at(1), BDL_DEV as u32, "started before the descriptor address was set");
    assert_eq!(s.at(2), (BDL_DEV >> 32) as u32, "started before the upper address half");
    assert_eq!(s.at(3) & 0xffff, (N_PERIODS - 1) as u32, "started before the last index");
}
