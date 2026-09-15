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

//! Which device on the bus ends up addressed.
//!
//! IC_TAR holds the address every subsequent transfer goes to. There is no
//! acknowledgement that it took and no way to tell from the transfer itself
//! that the wrong device answered, so a value that is off by a bit reads back
//! as a device that has started returning nonsense.

use crate::constants::IC_TAR;
use crate::model::live;
use crate::regs::Regs;
use crate::transaction::control::set_target;

const TOUCHPAD: u8 = 0x15;

#[test]
fn the_address_the_caller_asked_for_is_the_one_left_in_the_target_register() {
    let bar = live();
    assert!(set_target(Regs::new(bar.base()), TOUCHPAD).is_ok(), "target change refused");
    assert_eq!(bar.wrote32(IC_TAR as usize), TOUCHPAD as u32);
}

#[test]
fn only_the_seven_address_bits_reach_the_target_register() {
    /*
     * I2C addresses are seven bits wide and the bits above them are not spare
     * padding: on this core they select 10-bit addressing. Letting a caller's
     * stray high bit through does not address the wrong device, it puts the
     * controller into a different addressing protocol for every transfer that
     * follows.
     */
    let bar = live();
    assert!(set_target(Regs::new(bar.base()), 0x95).is_ok(), "target change refused");
    assert_eq!(bar.wrote32(IC_TAR as usize), 0x15);
}
