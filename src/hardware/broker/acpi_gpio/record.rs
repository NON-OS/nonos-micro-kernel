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

use crate::arch::x86_64::acpi::aml::GpioController;
use crate::hardware::broker::class::ids;
use crate::hardware::broker::device::{Bar, BarKind, BusKind, DeviceRecord};

/// Build an ACPI-bus DeviceRecord for a discovered GPIO community controller:
/// one MMIO bar over the community's register window so a driver capsule can
/// claim it and map the interrupt status registers through the `MkMmioMap`
/// grant path. The firmware `_UID` (the community index that a touchpad record
/// refers to) rides in `device`; `vendor` stays zero because an ACPI device
/// carries no PCI vendor id.
pub(super) fn device_record(ctl: &GpioController) -> DeviceRecord {
    let mut bars = [Bar::empty(); 6];
    bars[0] = Bar {
        base: ctl.mmio_base,
        size: ctl.mmio_size,
        kind: BarKind::Mmio as u8,
        flags: 0,
        aux: 0,
        _pad: [0; 2],
    };
    DeviceRecord {
        bus_kind: BusKind::Acpi as u8,
        class: ids::GPIO_CTRL,
        vendor: 0,
        device: (ctl.uid & 0xFFFF) as u16,
        bar_count: 1,
        bars,
        ..DeviceRecord::empty()
    }
}
