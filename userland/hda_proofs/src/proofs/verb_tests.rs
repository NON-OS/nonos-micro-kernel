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

//! Posting a verb and collecting its answer.
//!
//! This is the exchange a passive window cannot serve, and the reason this
//! capsule had no host coverage before. The driver writes a command into the
//! ring, advances CORBWP and then waits on RIRBWP, which is the controller's
//! register and never the driver's. Nothing in memory ever moves it, so every
//! path past this point was reachable only by booting a machine that has an
//! audio controller in it. A model on another thread advances it instead.

use nonos_devmodel::run;

use crate::constants::CORBWP;
use crate::controller::verb;
use crate::model::{corb_engine, rings, window};
use crate::regs::Regs;

pub const RESPONSE: u32 = 0x1234_5678;
pub const CMD: u32 = 0x0010_f000;

#[test]
fn a_running_controller_answers_a_posted_command() {
    let bar = window();
    let (corb, rirb) = rings(RESPONSE);
    let _controller = run(&bar, corb_engine);
    let mut wp = 0u16;
    let got = verb::send(Regs::new(bar.base()), corb.base(), rirb.base(), &mut wp, CMD);
    assert!(got == Ok(RESPONSE), "the response was not read from the answered slot");
    assert_eq!(wp, 1, "the driver's shadow write pointer did not follow the controller");
    assert_eq!(bar.wrote16(CORBWP as usize), 1, "the controller was never told to fetch");
}

#[test]
fn the_first_command_is_posted_one_slot_ahead_of_where_the_controller_reads() {
    /*
     * Both pointers start at zero and the controller consumes the slot after
     * the one it last read. A command left in slot zero is fetched only after
     * a full lap of the ring, so the first verb of every boot hangs.
     */
    let bar = window();
    let (corb, rirb) = rings(RESPONSE);
    let _controller = run(&bar, corb_engine);
    let mut wp = 0u16;
    assert!(verb::send(Regs::new(bar.base()), corb.base(), rirb.base(), &mut wp, CMD).is_ok());
    assert_eq!(corb.wrote32(4), CMD, "the command is not in slot one");
    assert_eq!(corb.wrote32(0), 0, "slot zero was written over");
}
