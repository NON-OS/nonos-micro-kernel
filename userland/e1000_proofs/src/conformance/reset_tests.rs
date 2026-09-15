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

//! The reset handshake: requested, waited for, then the part quiesced and
//! the link set up.

use super::model::{live_part, window};
use crate::constants::regs::{REG_CTRL, REG_IMC};
use crate::constants::status::{CTRL_ASDE, CTRL_LRST, CTRL_RST, CTRL_SLU};
use crate::init::reset_run;
use crate::regs::Regs;

#[test]
fn reset_is_requested_waited_for_then_interrupts_masked_and_the_link_set_up() {
    let bar = window();
    let _part = live_part(&bar);
    bar.present32(REG_CTRL, CTRL_LRST);
    reset_run(&Regs::new(bar.base())).expect("the part completed the reset");
    let ctrl = bar.wrote32(REG_CTRL);
    assert_eq!(ctrl & CTRL_RST, 0, "reset is over");
    assert_eq!(ctrl & CTRL_LRST, 0, "the link is out of reset");
    assert_eq!(ctrl & (CTRL_SLU | CTRL_ASDE), CTRL_SLU | CTRL_ASDE, "link up, speed detected");
    assert_eq!(bar.wrote32(REG_IMC), 0xFFFF_FFFF, "every interrupt cause masked");
}

#[test]
fn a_part_that_never_completes_reset_is_given_up_on() {
    let bar = window();
    let err = reset_run(&Regs::new(bar.base())).err();
    assert_eq!(err, Some("CTRL.RST did not self-clear"));
}
