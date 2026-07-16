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
pub(super) const MAX_DEVICES: usize = 128;
pub(super) const PCI_CLASS_SERIAL_BUS: u8 = 0x0c;
pub(super) const ACPI_LPSS_FAMILY: &str = "acpi-lpss";
pub(super) const CLASS_I2C_HID: u32 = 0x0041;

/// Upper bound on host controllers probed in one bring-up. Gemini Lake exposes
/// eight LPSS I2C functions; other platforms fewer.
pub const MAX_CONTROLLERS: usize = 8;

#[derive(Clone, Copy, Default)]
pub struct Found {
    pub device_id: u64,
    pub irq_line: u8,
    pub bar0_size: u64,
    pub pci_device: u16,
    pub clock_hz: u32,
    pub family: &'static str,
    pub is_acpi: bool,
}

/// An ACPI-declared HID device: its 7-bit I2C address, used to probe which
/// controller actually answers it, and the controller index the firmware named
/// in its `_CRS` ResourceSource (carried as index+1 in `pci_progif`, zero when
/// unnamed).
#[derive(Clone, Copy, Default)]
pub struct AcpiTouchpad {
    pub addr: u8,
    pub controller_idx: Option<u8>,
}

/// Upper bound on ACPI HID candidates considered. Multi-SKU firmware declares
/// one device per possible pad; only the fitted one answers.
pub const MAX_TARGETS: usize = 4;
