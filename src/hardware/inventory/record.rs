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

use super::classify::classify_family;
use super::driver::family_driver;
use super::family::HardwareFamily;
use super::missing::missing_path;
use super::state::SupportState;
use super::support::support_state;
use crate::hardware::broker::DeviceRecord;

#[derive(Debug, Clone, Copy)]
pub struct InventoryRecord {
    pub device_id: u64,
    pub vendor: u16,
    pub device: u16,
    pub pci_class: u8,
    pub pci_subclass: u8,
    pub pci_progif: u8,
    pub family: HardwareFamily,
    pub driver: Option<&'static str>,
    pub state: SupportState,
    pub missing: Option<&'static str>,
}

impl InventoryRecord {
    pub fn from_device_record(rec: &DeviceRecord) -> Self {
        let family = classify_family(
            rec.pci_class,
            rec.pci_subclass,
            rec.pci_progif,
            rec.vendor,
            rec.device,
        );
        Self {
            device_id: rec.device_id,
            vendor: rec.vendor,
            device: rec.device,
            pci_class: rec.pci_class,
            pci_subclass: rec.pci_subclass,
            pci_progif: rec.pci_progif,
            family,
            driver: family_driver(family),
            state: support_state(family),
            missing: missing_path(family),
        }
    }
}
