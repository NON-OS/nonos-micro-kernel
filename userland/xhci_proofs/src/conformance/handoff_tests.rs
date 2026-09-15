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

//! Taking the controller from firmware.

use nonos_devmodel::run;

use super::devices::cooperative_firmware;
use super::model::{controller, with_legacy_cap, BIOS_OWNED, LEGACY, OS_OWNED};
use crate::controller::legacy_handoff;

#[test]
fn ownership_passes_when_firmware_releases_and_smis_are_masked() {
    let bar = with_legacy_cap(true);
    let _fw = run(&bar, cooperative_firmware);
    legacy_handoff(bar.base());
    let sup = bar.wrote32(LEGACY);
    assert_ne!(sup & OS_OWNED, 0);
    assert_eq!(sup & BIOS_OWNED, 0);
    /*
     * USBLEGCTLSTS: the enable bits are 0, 4, 13, 14 and 15 and must read
     * back clear; bits 1..3, 5..12 and 17..19 are reserved or read-only and
     * are preserved; 29..31 are RW1C status and are written one to clear.
     */
    let ctl = bar.wrote32(LEGACY + 4);
    let enables = 1 | (1 << 4) | (1 << 13) | (1 << 14) | (1 << 15);
    assert_eq!(ctl & enables, 0, "every SMI enable is cleared");
    assert_eq!(ctl & (0xFF << 5), 0xFF << 5, "reserved bits are preserved");
    assert_eq!(ctl & (0x7 << 29), 0x7 << 29, "pending SMI status is acknowledged");
}

#[test]
fn firmware_that_never_releases_is_forced_off_after_the_timeout() {
    let bar = with_legacy_cap(true);
    legacy_handoff(bar.base());
    assert_eq!(bar.wrote32(LEGACY) & BIOS_OWNED, 0, "the BIOS bit is forced down");
    assert_ne!(bar.wrote32(LEGACY) & OS_OWNED, 0);
}

#[test]
fn a_controller_without_a_legacy_capability_is_left_alone() {
    let bar = controller(0);
    bar.present32(LEGACY, 0xDEAD_BEEF);
    legacy_handoff(bar.base());
    assert_eq!(bar.wrote32(LEGACY), 0xDEAD_BEEF);
}
