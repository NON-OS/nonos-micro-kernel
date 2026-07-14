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

pub mod i2c;
pub mod irq;
pub mod lookup;
pub mod pci;

pub use i2c::{
    classify_hid_device, enumerate_i2c_hid_devices, find_touchpads, find_touchscreens,
    I2cHidDevice, I2cHidDeviceType,
};
pub use irq::{irq_to_gsi, is_irq_active_low, is_irq_level_triggered};
pub use lookup::{
    enabled_processor_count, get_hpet_base, get_ioapic_addresses, get_ioapic_for_gsi,
    get_lapic_base, get_pcie_ecam, has_8042, has_legacy_pics, numa_domains, processor_count,
};
pub use pci::{enumerate_pci_devices, enumerate_pci_raw, PciDevice};
