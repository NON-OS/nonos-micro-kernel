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

use super::pci_index;
use crate::drivers::pci::ConfigSpace;

// Bring a claimed PCI device into power state D0 before its driver touches MMIO.
// Intel LPSS functions (I2C, UART) can be left in D3 by firmware, and in D3 their
// MMIO register window is dead, so every transfer silently fails. The monolithic
// driver used to set D0 itself; a sandboxed capsule cannot write the power
// management register through the broker, so the broker does it on claim. A no-op
// for non-PCI (ACPI/legacy) devices, which are absent from the PCI index.
pub(super) fn power_on_device(device_id: u64) {
    if let Some(handle) = pci_index::lookup(device_id) {
        let _ = ConfigSpace::new(handle.address).set_power_state_d0();
    }
}
