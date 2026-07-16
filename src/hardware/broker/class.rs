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

// Class IDs surfaced through `MkDeviceList`. These match
// `abi/driver_broker_abi.md`.

#[allow(non_upper_case_globals)]
pub(in crate::hardware::broker) mod ids {
    pub(in crate::hardware::broker) const RNG: u32 = 0x0001;
    pub(in crate::hardware::broker) const BLOCK: u32 = 0x0010;
    pub(in crate::hardware::broker) const NETWORK: u32 = 0x0020;
    pub(in crate::hardware::broker) const DISPLAY: u32 = 0x0030;
    pub(in crate::hardware::broker) const INPUT: u32 = 0x0040;
    /// Subset of INPUT: an ACPI-declared I2C-HID touchpad, carrying its I2C
    /// slave address and HID descriptor register for the HID driver to bind
    /// without probing.
    pub(in crate::hardware::broker) const I2C_HID: u32 = 0x0041;
    pub(in crate::hardware::broker) const AUDIO: u32 = 0x0050;
    pub(in crate::hardware::broker) const SERIAL: u32 = 0x0060;
    pub(in crate::hardware::broker) const USB_HOST: u32 = 0x0070;
    /// Subset of USB_HOST: a controller advertising xHCI prog-if
    /// (0x30). Older UHCI/OHCI/EHCI controllers stay on the
    /// generic USB_HOST id. Userland discovery matches on this
    /// id so it never tries to drive a non-xHCI USB host.
    pub(in crate::hardware::broker) const USB_HOST_XHCI: u32 = 0x0071;
    /// A platform GPIO community controller enumerated from ACPI. The input
    /// driver claims the community a touchpad's GpioInt names and maps its
    /// MMIO window to poll the pad's interrupt status register.
    pub(in crate::hardware::broker) const GPIO_CTRL: u32 = 0x0080;
    pub(in crate::hardware::broker) const OTHER: u32 = 0xFFFF;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Class(pub u32);

impl Class {
    pub const fn id(self) -> u32 {
        self.0
    }
}

// Maps a PCI class/subclass/progif to a broker class id. Anything we
// do not classify lands in `OTHER` so the table still surfaces it.
pub fn classify_pci(class: u8, subclass: u8, progif: u8) -> Class {
    Class(match (class, subclass) {
        // Mass storage
        (0x01, 0x06) => ids::BLOCK, // SATA / AHCI
        (0x01, 0x08) => ids::BLOCK, // NVMe
        (0x01, _) => ids::BLOCK,
        // Network
        (0x02, _) => ids::NETWORK,
        // Display
        (0x03, _) => ids::DISPLAY,
        // Multimedia (audio class 0x04, subclass 0x01 / 0x03)
        (0x04, 0x01) => ids::AUDIO,
        (0x04, 0x03) => ids::AUDIO,
        // Input
        (0x09, _) => ids::INPUT,
        // Serial
        (0x07, 0x00) => ids::SERIAL,
        // USB host. xHCI advertises prog-if 0x30; UHCI/OHCI/EHCI
        // surface as the generic id since this kernel does not
        // ship drivers for them.
        (0x0c, 0x03) if progif == 0x30 => ids::USB_HOST_XHCI,
        (0x0c, 0x03) => ids::USB_HOST,
        _ => ids::OTHER,
    })
}
