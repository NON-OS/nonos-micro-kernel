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

//! A controller service standing behind the HID driver's IPC calls, with a
//! touchpad on the modelled bus.

use i2c_transfer_proofs::bench::{driver, GEMINI_LAKE_HZ};
use i2c_transfer_proofs::regs::{self, Attached};
use nonos_i2cmodel::{touchpad, Bus, Config, Designware, HidOverI2c, Shared};
use nonos_libc::{serve, Served};

use super::service::service;

pub const PAD: u8 = 0x15;
pub const HID_DESC_REG: u16 = 0x0001;
pub const MAX_INPUT: u16 = 2 + touchpad::TOUCH_REPORT_LEN;

pub struct Rig {
    pub pad: Shared<HidOverI2c>,
    _served: Served,
    _attached: Attached,
}

pub fn pad() -> HidOverI2c {
    HidOverI2c::new(PAD, touchpad::report_descriptor(), MAX_INPUT, 0x04F3, 0x3028)
}

/// The pad where the firmware said it is.
pub fn rig() -> Rig {
    rig_with(pad(), Some((PAD, HID_DESC_REG)))
}

/// `device` on the bus, and `hint` as what the platform's ACPI named: the
/// device address and its descriptor register, or nothing at all.
pub fn rig_with(device: HidOverI2c, hint: Option<(u8, u16)>) -> Rig {
    let pad = Shared::new(device);
    let attached = regs::attach(Designware::new(Config::LPSS, Bus::default().with(pad.clone())));
    let controller = driver(GEMINI_LAKE_HZ, hint.map_or(0, |h| h.0)).expect("controller bring-up");
    let served = serve(move |req, resp| service(&controller, hint, req, resp));
    Rig { pad, _served: served, _attached: attached }
}

/// The events of one kind among those posted, in order. The raw-report
/// signal the driver posts alongside every frame is not an input event and
/// is what this leaves out.
pub fn of_kind(events: &[nonos_libc::InputEvent], kind: u16) -> Vec<nonos_libc::InputEvent> {
    events.iter().copied().filter(|e| e.kind == kind).collect()
}
