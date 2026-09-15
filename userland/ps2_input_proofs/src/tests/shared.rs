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

//! Attaching a controller behind the shim's port calls while the test keeps
//! a handle to look at it afterwards.

use std::cell::RefCell;
use std::rc::Rc;

use nonos_libc::{attach, Attached, DeviceRecord, Port, BUS_KIND_ACPI};

use super::controller::{Controller, Keyboard, Mouse, MouseKind};
use crate::constants::{
    PNP_DEVICE_PS2_AUX, PNP_DEVICE_PS2_KBD, PNP_VENDOR_PS2_AUX, PNP_VENDOR_PS2_KBD,
};

pub struct Shared(Rc<RefCell<Controller>>);

impl Port for Shared {
    fn read(&mut self, offset: u16) -> Option<u8> {
        Some(self.0.borrow_mut().read(offset))
    }

    fn write(&mut self, offset: u16, value: u8) -> bool {
        self.0.borrow_mut().write(offset, value);
        true
    }
}

pub type Handle = Rc<RefCell<Controller>>;

pub fn attached(controller: Controller) -> (Handle, Attached) {
    let handle = Rc::new(RefCell::new(controller));
    let guard = attach(Box::new(Shared(Rc::clone(&handle))));
    (handle, guard)
}

/// A controller as firmware commonly leaves it: interrupts off, first port
/// disabled, aux port disabled.
pub const FIRMWARE_CONFIG: u8 = 0x30;

pub fn machine(keyboard: Keyboard, mouse: MouseKind) -> (Handle, Attached) {
    attached(Controller::new(FIRMWARE_CONFIG, keyboard, Mouse::new(mouse)))
}

/// The two records the broker lists for a machine with both ports wired.
pub const KBD: DeviceRecord = DeviceRecord {
    device_id: 40,
    bus_kind: BUS_KIND_ACPI,
    vendor: PNP_VENDOR_PS2_KBD,
    device: PNP_DEVICE_PS2_KBD,
    bar_count: 1,
    irq_line: 1,
};
pub const AUX: DeviceRecord = DeviceRecord {
    device_id: 41,
    bus_kind: BUS_KIND_ACPI,
    vendor: PNP_VENDOR_PS2_AUX,
    device: PNP_DEVICE_PS2_AUX,
    bar_count: 0,
    irq_line: 12,
};
