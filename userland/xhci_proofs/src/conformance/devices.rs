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

//! What answers from the other side of the window: a host controller, and
//! the firmware that owned it first. The device on a port is in `port_device`.

use nonos_devmodel::FakeBar;

use super::model::{BIOS_OWNED, CAP_LEN, LEGACY, OS_OWNED};
use crate::constants::{USBCMD, USBCMD_HCRST, USBCMD_RUN, USBSTS, USBSTS_CNR, USBSTS_HCH};

/// A conforming host controller: halted exactly when not running, ready once
/// a requested reset has been performed, and reset completes on the next look.
pub fn host_controller(bar: &FakeBar) {
    let cmd = bar.wrote32(CAP_LEN + USBCMD as usize);
    let mut sts = bar.wrote32(CAP_LEN + USBSTS as usize);
    if cmd & USBCMD_HCRST != 0 {
        bar.present32(CAP_LEN + USBCMD as usize, cmd & !USBCMD_HCRST);
        sts &= !USBSTS_CNR;
    }
    sts = if cmd & USBCMD_RUN != 0 { sts & !USBSTS_HCH } else { sts | USBSTS_HCH };
    bar.present32(CAP_LEN + USBSTS as usize, sts);
}

/// A controller that acknowledges nothing: whatever the driver writes stays
/// written and no status bit ever moves on its own.
pub fn dead_controller(_bar: &FakeBar) {}

/// Firmware that releases the controller as soon as the OS asks for it. It
/// writes only when there is something to release, so the driver's own write
/// is never overwritten before firmware has seen it.
pub fn cooperative_firmware(bar: &FakeBar) {
    let v = bar.wrote32(LEGACY);
    if v & OS_OWNED != 0 && v & BIOS_OWNED != 0 {
        bar.present32(LEGACY, v & !BIOS_OWNED);
    }
}
