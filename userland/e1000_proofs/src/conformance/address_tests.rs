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

//! The station address: drawn, local, programmed into the receive address
//! registers, and never the one in the EEPROM.

use nonos_libc::entropy;
use nonos_mac::is_local_unicast;

use super::model::{receive_address, window, EEPROM};
use crate::constants::regs::{MTA_ENTRY_COUNT, REG_MTA_BASE, REG_RAH0};
use crate::constants::status::RAH_AV;
use crate::init::{draw, mac_program};
use crate::regs::Regs;

#[test]
fn the_drawn_address_is_locally_administered_unicast_and_not_the_eeprom_one() {
    let _turn = entropy(true);
    let mac = draw().expect("an address is drawn");
    assert!(is_local_unicast(&mac));
    assert_ne!(mac, EEPROM);
}

#[test]
fn without_entropy_no_address_is_drawn() {
    let _turn = entropy(false);
    assert_eq!(draw().err(), Some("no entropy for station address"));
}

#[test]
fn the_receive_address_is_programmed_valid_and_the_multicast_table_cleared() {
    let bar = window();
    for i in 0..MTA_ENTRY_COUNT {
        bar.present32(REG_MTA_BASE + i * 4, 0xFFFF_FFFF);
    }
    let mac = [0x02, 0x11, 0x22, 0x33, 0x44, 0x55];
    mac_program(&Regs::new(bar.base()), &mac);
    assert_eq!(receive_address(&bar), mac);
    assert_ne!(bar.wrote32(REG_RAH0) & RAH_AV, 0, "the entry is marked valid");
    for i in 0..MTA_ENTRY_COUNT {
        assert_eq!(bar.wrote32(REG_MTA_BASE + i * 4), 0, "multicast entry {i} cleared");
    }
}
