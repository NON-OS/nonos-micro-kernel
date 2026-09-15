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

//! The broker's types as the driver reads them, and the grant numbers.

pub const BUS_KIND_ACPI: u8 = 2;
pub const PIO_GRANT: u64 = 7;
/// An interrupt grant is the line number plus this, so the two lines the
/// driver opens are told apart in what it acknowledged.
pub const IRQ_GRANT_BASE: u64 = 100;

/// The fields of the broker's record the driver reads.
#[derive(Clone, Copy, Default)]
pub struct DeviceRecord {
    pub device_id: u64,
    pub bus_kind: u8,
    pub vendor: u16,
    pub device: u16,
    pub bar_count: u8,
    pub irq_line: u8,
}

impl DeviceRecord {
    pub const fn empty() -> Self {
        Self { device_id: 0, bus_kind: 0, vendor: 0, device: 0, bar_count: 0, irq_line: 0 }
    }
}

#[repr(C)]
pub struct PioGrantOut {
    pub port_base: u16,
    pub port_count: u16,
    pub _pad: u32,
    pub grant_id: u64,
}

pub struct IrqBindOut {
    pub grant_id: u64,
    pub vector: u64,
}
