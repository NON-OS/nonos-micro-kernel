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

use super::capabilities::*;
use super::classes::*;
use super::pcie::*;
use super::registers::CFG_BAR0;

#[inline]
pub const fn bar_offset(index: u8) -> u16 {
    CFG_BAR0 + (index as u16 * 4)
}

pub fn class_name(class: u8) -> &'static str {
    match class {
        CLASS_UNCLASSIFIED => "Unclassified",
        CLASS_MASS_STORAGE => "Mass Storage",
        CLASS_NETWORK => "Network",
        CLASS_DISPLAY => "Display",
        CLASS_MULTIMEDIA => "Multimedia",
        CLASS_MEMORY => "Memory",
        CLASS_BRIDGE => "Bridge",
        CLASS_SIMPLE_COMM => "Simple Communication",
        CLASS_BASE_PERIPHERAL => "Base Peripheral",
        CLASS_INPUT => "Input",
        CLASS_DOCKING => "Docking Station",
        CLASS_PROCESSOR => "Processor",
        CLASS_SERIAL_BUS => "Serial Bus",
        CLASS_WIRELESS => "Wireless",
        CLASS_INTELLIGENT_IO => "Intelligent I/O",
        CLASS_SATELLITE_COMM => "Satellite Communication",
        CLASS_ENCRYPTION => "Encryption",
        CLASS_SIGNAL_PROCESSING => "Signal Processing",
        CLASS_PROCESSING_ACCELERATOR => "Processing Accelerator",
        CLASS_NON_ESSENTIAL => "Non-Essential",
        CLASS_COPROCESSOR => "Coprocessor",
        _ => "Unknown",
    }
}

pub fn capability_name(id: u8) -> &'static str {
    match id {
        CAP_ID_PM => "Power Management",
        CAP_ID_AGP => "AGP",
        CAP_ID_VPD => "Vital Product Data",
        CAP_ID_SLOT_ID => "Slot ID",
        CAP_ID_MSI => "MSI",
        CAP_ID_CHSWP => "CompactPCI Hot Swap",
        CAP_ID_PCIX => "PCI-X",
        CAP_ID_HT => "HyperTransport",
        CAP_ID_VNDR => "Vendor Specific",
        CAP_ID_DBG => "Debug Port",
        CAP_ID_CCRC => "CompactPCI CRC",
        CAP_ID_SHPC => "Hot Plug",
        CAP_ID_SSVID => "Subsystem Vendor ID",
        CAP_ID_AGP3 => "AGP 3.0",
        CAP_ID_SECDEV => "Secure Device",
        CAP_ID_PCIE => "PCI Express",
        CAP_ID_MSIX => "MSI-X",
        CAP_ID_SATA => "SATA",
        CAP_ID_AF => "Advanced Features",
        CAP_ID_EA => "Enhanced Allocation",
        CAP_ID_FPB => "Flattening Portal Bridge",
        _ => "Unknown",
    }
}

pub fn pcie_link_speed_str(speed: u8) -> &'static str {
    match speed {
        PCIE_LINK_SPEED_2_5GT => "2.5 GT/s (Gen1)",
        PCIE_LINK_SPEED_5GT => "5 GT/s (Gen2)",
        PCIE_LINK_SPEED_8GT => "8 GT/s (Gen3)",
        PCIE_LINK_SPEED_16GT => "16 GT/s (Gen4)",
        PCIE_LINK_SPEED_32GT => "32 GT/s (Gen5)",
        PCIE_LINK_SPEED_64GT => "64 GT/s (Gen6)",
        _ => "Unknown",
    }
}
