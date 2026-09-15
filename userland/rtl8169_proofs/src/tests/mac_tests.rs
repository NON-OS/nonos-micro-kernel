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

//! The station address: drawn, local, programmed under the lock, and never the
//! factory one.

use nonos_libc::entropy;
use nonos_mac::is_local_unicast;

use super::model::{idr, window, FACTORY};
use crate::constants::regs::REG_CFG9346;
use crate::init::mac_program;
use crate::regs::Regs;

#[test]
fn the_programmed_address_is_drawn_locally_administered_and_not_the_factory_one() {
    let _turn = entropy(true);
    let bar = window();
    let mac = mac_program(&Regs::new(bar.base())).expect("an address is programmed");
    assert_ne!(mac, FACTORY);
    assert!(is_local_unicast(&mac), "locally administered, unicast");
    assert_eq!(idr(&bar), mac, "the IDR bytes hold the drawn address");
    assert_eq!(bar.wrote8(REG_CFG9346), 0x00, "9346CR back to normal mode: the lock");
}

#[test]
fn without_entropy_nothing_is_programmed_and_the_factory_address_is_not_used() {
    let _turn = entropy(false);
    let bar = window();
    let result = mac_program(&Regs::new(bar.base()));
    assert!(result.is_err(), "no entropy is a refusal, not a fallback");
    assert_eq!(idr(&bar), FACTORY, "the IDR bytes were not touched");
    assert_eq!(bar.wrote8(REG_CFG9346), 0x00, "the lock was never opened");
}
