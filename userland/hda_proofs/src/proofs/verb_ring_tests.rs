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

//! Staying inside the ring that was actually allocated.
//!
//! The write pointer is masked, not bounded by the buffer: the ring is 256
//! entries because the driver told the controller so and allocated to match.
//! A pointer that walks past 255 addresses memory beyond the allocation from
//! both sides at once, the driver by store and the controller by DMA, and
//! neither notices. Wrapping is also the only thing that lets a boot send more
//! than 255 verbs, which a codec graph walk on a real part comfortably does.

use nonos_devmodel::run;

use crate::constants::{RIRBSTS, RIRBSTS_INTFL};
use crate::controller::verb;
use crate::model::{corb_engine, rings, window};
use crate::proofs::verb_tests::{CMD, RESPONSE};
use crate::regs::Regs;

#[test]
fn the_write_pointer_wraps_within_the_ring_rather_than_running_off_the_end() {
    let bar = window();
    let (corb, rirb) = rings(RESPONSE);
    let _controller = run(&bar, corb_engine);
    let regs = Regs::new(bar.base());
    let mut wp = 0u16;
    for i in 0..256u32 {
        assert!(verb::send(regs, corb.base(), rirb.base(), &mut wp, CMD + i).is_ok(), "verb {i}");
    }
    assert_eq!(wp, 0, "the pointer left the 256-entry ring the controller was given");
    assert_eq!(corb.wrote32(0), CMD + 255, "the last command did not wrap into slot zero");
    assert_eq!(corb.wrote32(4), CMD, "the first command was overwritten early");
}

#[test]
fn the_response_interrupt_flag_is_acknowledged_after_every_answer() {
    /*
     * RIRBSTS latches and is cleared by writing a one back. Left set, the
     * controller raises no further response interrupt, and the poll loop that
     * drives the capsule's IRQ path stops seeing anything arrive.
     */
    let bar = window();
    let (corb, rirb) = rings(RESPONSE);
    let _controller = run(&bar, corb_engine);
    let mut wp = 0u16;
    assert!(verb::send(Regs::new(bar.base()), corb.base(), rirb.base(), &mut wp, CMD).is_ok());
    assert_eq!(bar.wrote8(RIRBSTS as usize), RIRBSTS_INTFL);
}
