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

/// A touchpad (or other I2C-HID peripheral) enumerated from the ACPI AML
/// namespace. The fields carry the firmware-declared parameters a driver needs
/// to bind the device without blind probing.
#[derive(Debug, Clone, Copy)]
pub struct I2cHidDevice {
    /// 7-bit I2C slave address from the I2cSerialBus descriptor. 10-bit
    /// addressing is out of scope; the raw 16-bit field is masked to 7 bits.
    pub slave_addr: u16,
    /// The GPIO pin number carrying the device interrupt, taken from the first
    /// entry of the GpioInt descriptor's pin table.
    pub gpio_pin: u16,
    /// HID descriptor register offset. ACPI declares this via the device's
    /// _DSM method (not parsed by this bounded extractor), so it defaults to
    /// 0x0001, the value used by the vast majority of I2C-HID touchpads.
    pub hid_desc_reg: u16,
    /// The seven-character EISAID (or the first eight bytes of a string _HID)
    /// that matched, padded with zero bytes. Retained for diagnostics.
    pub hid: [u8; 8],
    /// True when a GpioInt descriptor was found and `gpio_pin` is meaningful.
    pub has_gpio: bool,
    /// The final NameSeg of the I2cSerialBus ResourceSource, i.e. the ACPI name
    /// of the host controller the device hangs off (for example `I2C4`). Zero
    /// when the descriptor carried no ResourceSource. This is what selects the
    /// correct controller when several I2C hosts are present.
    pub controller: [u8; 4],
}

impl I2cHidDevice {
    /// Default HID descriptor register when _DSM is not evaluated.
    pub const DEFAULT_HID_DESC_REG: u16 = 0x0001;

    pub fn new(hid: [u8; 8]) -> Self {
        Self {
            slave_addr: 0,
            gpio_pin: 0,
            hid_desc_reg: Self::DEFAULT_HID_DESC_REG,
            hid,
            has_gpio: false,
            controller: [0; 4],
        }
    }
}

/// An LPSS-style I2C host controller enumerated from the ACPI namespace by its
/// `_HID`, with the MMIO window and interrupt recovered from its `_CRS`. On a
/// modern laptop the controller is an ACPI device (for example INT33C3 or
/// AMDI0010), not a PCI function, so PCI enumeration never finds it and the
/// touchpad hanging off it stays unreachable until the controller is found
/// this way.
#[derive(Debug, Clone, Copy)]
pub struct LpssController {
    /// Base of the controller's MMIO register window, from the Memory32Fixed
    /// descriptor in `_CRS`.
    pub mmio_base: u64,
    /// Length of that MMIO window in bytes.
    pub mmio_size: u32,
    /// The controller interrupt number, from the Extended Interrupt descriptor.
    pub irq: u32,
    /// True when an Extended Interrupt descriptor was found and `irq` is
    /// meaningful.
    pub has_irq: bool,
    /// The eight-byte `_HID` that matched, retained for diagnostics.
    pub hid: [u8; 8],
}

impl LpssController {
    pub fn new(hid: [u8; 8]) -> Self {
        Self { mmio_base: 0, mmio_size: 0, irq: 0, has_irq: false, hid }
    }

    /// A controller record is usable only once its MMIO window is known; the
    /// interrupt is optional because the transfer engine can poll.
    pub fn is_valid(&self) -> bool {
        self.mmio_base != 0 && self.mmio_size != 0
    }
}
