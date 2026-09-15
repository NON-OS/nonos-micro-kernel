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

//! The whole bring-up: on the air under a drawn address, or not at all.

use nonos_libc::entropy;

use super::memory::Memory;
use super::model::{live_part, receive_address, window, EEPROM};
use crate::constants::regs::{REG_RCTL, REG_TCTL};
use crate::constants::status::{RCTL_EN, TCTL_EN};
use crate::init::bring_up;

/*
 * What is held here is the state the part is left in. The order of the
 * enable against the address write inside one bring-up is not observable
 * from a window that reads back final state, and a part sampling from another
 * thread misses a gap of a few instructions; that order is a property of the
 * source, not of this harness.
 */
#[test]
fn with_entropy_the_card_ends_up_filtering_on_the_drawn_address_and_enabled() {
    let _turn = entropy(true);
    let bar = window();
    let _part = live_part(&bar);
    let mut mem = Memory::new();
    let mut driver = mem.driver(&bar);
    bring_up(&mut driver).expect("brought up");
    assert_eq!(receive_address(&bar), driver.mac, "the part filters on the drawn address");
    assert_ne!(driver.mac, EEPROM);
    assert_ne!(bar.wrote32(REG_RCTL) & RCTL_EN, 0);
    assert_ne!(bar.wrote32(REG_TCTL) & TCTL_EN, 0);
}

#[test]
fn without_entropy_the_card_is_reset_and_left_off_the_air() {
    let _turn = entropy(false);
    let bar = window();
    let _part = live_part(&bar);
    let mut mem = Memory::new();
    let mut driver = mem.driver(&bar);
    let err = bring_up(&mut driver).err();
    assert_eq!(err, Some("no entropy for station address"));
    assert_eq!(receive_address(&bar), EEPROM, "the EEPROM address was left alone");
    assert_eq!(bar.wrote32(REG_RCTL), 0, "the receiver was never enabled");
    assert_eq!(bar.wrote32(REG_TCTL), 0, "nor the transmitter");
}
