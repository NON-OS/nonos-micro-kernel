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

extern crate alloc;

use alloc::vec::Vec;

use crate::drivers::pci::types::PciDevice;
use crate::hardware::broker::DeviceRecord;

use super::pci_record::record_from_pci;
use super::state::TABLE;
use crate::hardware::broker::pci_index::{self, PciHandle};

pub fn init_from_pci(devices: &[PciDevice]) {
    let mut records: Vec<DeviceRecord> = Vec::with_capacity(devices.len());
    let mut handles: Vec<PciHandle> = Vec::with_capacity(devices.len());
    for (idx, dev) in devices.iter().enumerate() {
        records.push(record_from_pci(idx as u64, dev));
        handles.push(PciHandle {
            device_id: idx as u64,
            address: dev.address,
            bars: dev.bars,
            msix: dev.msix,
        });
    }
    *TABLE.write() = records;
    pci_index::install(handles);
}

pub fn register_platform_device(mut record: DeviceRecord) -> u64 {
    let mut table = TABLE.write();
    let next = table.iter().map(|r| r.device_id).max().map(|m| m + 1);
    record.device_id = next.unwrap_or(0x1_0000_0000);
    table.push(record);
    record.device_id
}
